use std::fs::File;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::database::Database;

/// 应用全局状态，通过 tauri::State 注入到 Command 中。
///
/// KbNote 删减后只剩真正被使用的字段：数据库句柄、数据目录、启动参数暂存、单实例锁。
/// 原 kb 的 ai_cancel / sync_* / reminder_notify / push_notify / vault / mcp_* 均已随
/// 对应功能移除（见 docs/strip-plan.md）。
pub struct AppState {
    pub db: Database,
    /// 数据根目录（默认 = app_data_dir；用户改自定义目录 / KB_DATA_DIR / 便携模式后为对应路径）
    /// 资产/db 都基于此路径
    pub data_dir: PathBuf,
    /// 启动时 argv 里的 .md 文件路径，等前端 mount 后 take 出来
    pub pending_open_md_path: Mutex<Option<String>>,
    /// 单实例守护锁文件句柄（保持存活以维持独占锁，进程退出时自动释放）
    _lock_file: Option<File>,
}

impl AppState {
    pub fn new(db: Database, data_dir: PathBuf, lock_file: Option<File>) -> Self {
        Self {
            db,
            data_dir,
            pending_open_md_path: Mutex::new(None),
            _lock_file: lock_file,
        }
    }
}
