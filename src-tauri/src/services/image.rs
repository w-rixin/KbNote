use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use base64::{engine::general_purpose::STANDARD, Engine as _};

/// 进程内递增计数器，保证同一毫秒内多次保存也不会冲突
static IMAGE_SEQ: AtomicU64 = AtomicU64::new(0);

use crate::database::Database;
use crate::error::AppError;
use crate::services::safe_filename;

/// 图片资产目录名（dev 模式加 dev- 前缀实现数据隔离）
const ASSETS_DIR_PROD: &str = "kb_assets";
const ASSETS_DIR_DEV: &str = "dev-kb_assets";
const IMAGES_DIR: &str = "images";
/// 加密图片落盘后缀。文件名形如 `20260101_xxx.png.enc`，
/// 内容为 `aead_encrypt(vault_key, raw_bytes)`。
/// 用后缀而非独立目录的好处：scan_orphans / delete_note_images 等扫盘逻辑零改动。
const ENC_SUFFIX: &str = ".enc";

/// 笔记加密态切换时的图片迁移方向

#[inline]
fn assets_dir_name() -> &'static str {
    if cfg!(debug_assertions) {
        ASSETS_DIR_DEV
    } else {
        ASSETS_DIR_PROD
    }
}

pub struct ImageService;

impl ImageService {
    /// 获取图片根目录: {app_data_dir}/{prefix}kb_assets/images/
    pub fn images_dir(app_data_dir: &Path) -> PathBuf {
        app_data_dir.join(assets_dir_name()).join(IMAGES_DIR)
    }

    /// 确保图片目录存在
    pub fn ensure_dir(app_data_dir: &Path) -> Result<PathBuf, AppError> {
        let dir = Self::images_dir(app_data_dir);
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    /// 从 base64 数据保存图片（用于粘贴/拖放）。按笔记 is_encrypted 自动路由到加/明文分支。
    ///
    /// 返回保存后的绝对路径（加密笔记返回的路径以 `.enc` 结尾）
    pub fn save_from_base64(
        db: &Database,
        app_data_dir: &Path,
        note_id: i64,
        file_name: &str,
        base64_data: &str,
    ) -> Result<String, AppError> {
        let data = STANDARD
            .decode(base64_data)
            .map_err(|e| AppError::Custom(format!("base64 解码失败: {}", e)))?;

        Self::save_bytes_routed(db, app_data_dir, note_id, file_name, &data)
    }

    /// 从本地文件路径复制图片（用于工具栏插入）。按笔记 is_encrypted 自动路由。
    ///
    /// 返回保存后的绝对路径（加密笔记返回的路径以 `.enc` 结尾）
    pub fn save_from_path(
        db: &Database,
        app_data_dir: &Path,
        note_id: i64,
        source_path: &str,
    ) -> Result<String, AppError> {
        let source = Path::new(source_path);
        if !source.exists() {
            return Err(AppError::NotFound(format!("文件不存在: {}", source_path)));
        }

        let file_name = source
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image.png");

        let data = std::fs::read(source)?;
        Self::save_bytes_routed(db, app_data_dir, note_id, file_name, &data)
    }

    /// 反查笔记 is_encrypted，加密笔记走 AEAD + `.enc`，否则走明文。
    /// 用户交互入口（粘贴/拖放/工具栏）应优先用这个。
    /// KbNote 不含笔记加密（T-007 保险库已随删减移除）：一律走明文落盘。
    /// 保留 `_db` 参数是为了不打断调用方签名（语义上仍表示"按笔记路由"）。
    pub fn save_bytes_routed(
        _db: &Database,
        app_data_dir: &Path,
        note_id: i64,
        file_name: &str,
        data: &[u8],
    ) -> Result<String, AppError> {
        Self::save_bytes_inner(app_data_dir, note_id, file_name, data, false)
    }

    /// 保存字节数据到文件（明文模式；导入流程用，新建笔记 is_encrypted 默认 false）
    ///
    /// 调用方若不能保证笔记非加密态，请改用 `save_bytes_routed`。
    pub fn save_bytes(
        app_data_dir: &Path,
        note_id: i64,
        file_name: &str,
        data: &[u8],
    ) -> Result<String, AppError> {
        Self::save_bytes_inner(app_data_dir, note_id, file_name, data, false)
    }

    /// 内部落盘实现：`encrypted` 决定文件名是否追加 `.enc`。
    ///
    /// 命名规则（保留原名风格）：
    ///   - 优先 `images/<note_id>/<原 stem>.<ext>`
    ///   - 已存在 + 内容相同 → 复用旧文件（同一张图反复粘贴不浪费盘）
    ///   - 已存在 + 内容不同 → 加 `-1` / `-2` / ... 后缀
    ///
    /// 加密模式下统一在最终落盘文件名后追加 `.enc`，让扫盘逻辑无需改动。
    /// 加密判重比对的是密文，所以同一明文每次加密 nonce 不同 → 必然加后缀，
    /// 这是预期行为（无法在不解密的情况下做语义判重）。
    fn save_bytes_inner(
        app_data_dir: &Path,
        note_id: i64,
        file_name: &str,
        data: &[u8],
        encrypted: bool,
    ) -> Result<String, AppError> {
        let note_dir = Self::images_dir(app_data_dir).join(note_id.to_string());

        // 拆分原文件名为 stem + ext
        let path = Path::new(file_name);
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("image");
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("png");

        // 加密时把 .enc 拼到 ext 上，让 save_unique 一次定位到最终落盘名
        let final_ext = if encrypted {
            format!("{}{}", ext, ENC_SUFFIX)
        } else {
            ext.to_string()
        };

        let file_path = safe_filename::save_unique(&note_dir, stem, &final_ext, data)?;

        // 触一下进程内序号，保留旧的全局原子计数语义（其他模块可能依赖统计行为）
        IMAGE_SEQ.fetch_add(1, Ordering::Relaxed);

        log::info!(
            "图片已保存{}: {}",
            if encrypted { "（加密）" } else { "" },
            file_path.display()
        );
        Ok(file_path.to_string_lossy().into_owned())
    }

    /// 给 `get_image_blob` Command 用：读取磁盘原始字节（无加密态）。
    /// 安全：路径校验留给 Command 层。
    pub fn read_for_render(path: &str) -> Result<Vec<u8>, AppError> {
        Ok(std::fs::read(path)?)
    }

    /// 删除笔记的所有图片
    pub fn delete_note_images(app_data_dir: &Path, note_id: i64) -> Result<(), AppError> {
        let note_dir = Self::images_dir(app_data_dir).join(note_id.to_string());
        if note_dir.exists() {
            std::fs::remove_dir_all(&note_dir)?;
            log::info!("已删除笔记 {} 的所有图片", note_id);
        }
        Ok(())
    }

    // 孤儿扫描已迁移到 services::orphan_scan（统一扫 5 类素材，
    // 并修复 trash 笔记 content / 加密笔记 content 的引用判定 BUG）
}
