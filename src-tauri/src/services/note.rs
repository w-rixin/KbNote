use crate::database::Database;
use crate::error::AppError;
use crate::models::{Note, NoteImageRef, NoteInput, NoteQuery, PageResult};

/// 单篇笔记最多回传的图片数，防一篇图墙笔记撑爆 IPC 载荷 / 刷屏 AI 回答下方。
const MAX_IMAGES_PER_NOTE: usize = 12;

/// 笔记服务
/// 从笔记内容中抽出 `kb-asset://` 本地资产相对路径（去重、保序）。
///
/// 原实现来自 `sync_v1::attachment_scan`（随同步模块删除），逻辑等价：
/// 扫到 `kb-asset://` 前缀后，取到空白 / 引号 / 右括号 / `>` / `?` 为止。
fn extract_local_refs(content: &str) -> Vec<String> {
    const PREFIX: &str = "kb-asset://";
    let mut out: Vec<String> = Vec::new();
    let mut rest = content;
    while let Some(i) = rest.find(PREFIX) {
        let tail = &rest[i + PREFIX.len()..];
        let end = tail
            .find(|c: char| c.is_whitespace() || matches!(c, '"' | '\'' | ')' | '>' | '?'))
            .unwrap_or(tail.len());
        let rel = &tail[..end];
        if !rel.is_empty() && !out.iter().any(|x| x == rel) {
            out.push(rel.to_string());
        }
        rest = &tail[end..];
    }
    out
}

pub struct NoteService;

impl NoteService {
    /// 抽取一批笔记 content 里的图片资源，给 AI 回答下方"溯源"挂缩略图用。
    ///
    /// 入参一般来自 AI message 的 references（引用了哪几篇笔记）。逐篇取 content，
    /// 复用 `extract_local_refs` 抽出所有本地资产相对路径，再按 `kb_assets/images/`
    /// 路径段过滤——只留图片，自然排除视频(`videos/`)、PDF(`pdfs/`)、附件(`attachments/`)。
    /// 加密笔记 content 是占位符，抽不到图；查不到的 note_id 静默跳过（可能已删）。
    pub fn images_for_notes(
        db: &Database,
        note_ids: &[i64],
    ) -> Result<Vec<NoteImageRef>, AppError> {
        let mut out = Vec::new();
        for &id in note_ids {
            // get_note 返回 Result<Option<Note>>：查不到（None）或出错都静默跳过
            let note = match db.get_note(id) {
                Ok(Some(n)) => n,
                _ => continue,
            };
            let images: Vec<String> = extract_local_refs(&note.content)
                .into_iter()
                // dev 前缀目录 `dev-kb_assets/images/` 同样含该子串，prod/dev 都覆盖
                .filter(|rel| rel.contains("kb_assets/images/"))
                .take(MAX_IMAGES_PER_NOTE)
                .collect();
            if !images.is_empty() {
                out.push(NoteImageRef {
                    note_id: id,
                    title: note.title,
                    images,
                });
            }
        }
        Ok(out)
    }

    /// 创建笔记
    pub fn create(db: &Database, input: &NoteInput) -> Result<Note, AppError> {
        if input.title.trim().is_empty() {
            return Err(AppError::InvalidInput("笔记标题不能为空".into()));
        }
        db.create_note(input)
    }

    /// 更新笔记
    ///
    /// 仅做 DB 写入。外部 .md 写回由前端在保存成功后**显式调用** `write_back_source_md`，
    /// 这样冲突状态可以直接返回给调用方，避免事件 + 状态机的复杂度。
    pub fn update(db: &Database, id: i64, input: &NoteInput) -> Result<Note, AppError> {
        if input.title.trim().is_empty() {
            return Err(AppError::InvalidInput("笔记标题不能为空".into()));
        }
        // 覆盖之前留一份旧正文（自带时间窗节流 + 内容去重，见 services::snapshot）。
        // 必须在 update_note 之前 —— 此刻库里还是上一版；也必须在这一层而不是 DAO 里，
        // 因为 capture_auto 会自己去 get_note，在 DAO 的锁内再调会把同一个 Mutex 锁死。
        crate::services::snapshot::capture_auto(db, id);
        db.update_note(id, input)
    }

    /// 批量移动笔记到指定文件夹；返回实际移动的条数
    ///
    /// - `folder_id = None` → 移到根目录
    /// - folder_id 的合法性由前端（`folderApi.list()`）保证；这里不再 round-trip 校验
    pub fn move_batch(
        db: &Database,
        ids: &[i64],
        folder_id: Option<i64>,
    ) -> Result<usize, AppError> {
        if ids.is_empty() {
            return Ok(0);
        }
        db.move_notes_batch(ids, folder_id)
    }

    /// 批量软删除（移入回收站）；返回实际标记删除的条数
    pub fn trash_batch(db: &Database, ids: &[i64]) -> Result<usize, AppError> {
        if ids.is_empty() {
            return Ok(0);
        }
        db.soft_delete_notes_batch(ids)
    }

    /// 批量给多篇笔记追加标签（不清除原有标签）；返回新增的关联条数
    pub fn add_tags_batch(
        db: &Database,
        note_ids: &[i64],
        tag_ids: &[i64],
    ) -> Result<usize, AppError> {
        if note_ids.is_empty() || tag_ids.is_empty() {
            return Ok(0);
        }
        db.add_tags_to_notes_batch(note_ids, tag_ids)
    }

    /// 删除笔记（永久删除，预留给未来使用）
    #[allow(dead_code)]
    pub fn delete(db: &Database, id: i64) -> Result<(), AppError> {
        let deleted = db.delete_note(id)?;
        if !deleted {
            return Err(AppError::NotFound(format!("笔记 {} 不存在", id)));
        }
        Ok(())
    }

    /// 获取单个笔记
    pub fn get(db: &Database, id: i64) -> Result<Note, AppError> {
        db.get_note(id)?
            .ok_or_else(|| AppError::NotFound(format!("笔记 {} 不存在", id)))
    }

    /// 切换笔记置顶状态
    pub fn toggle_pin(db: &Database, id: i64) -> Result<bool, AppError> {
        db.toggle_pin(id)
    }

    /// 移动笔记到文件夹
    pub fn move_to_folder(
        db: &Database,
        note_id: i64,
        folder_id: Option<i64>,
    ) -> Result<(), AppError> {
        db.move_note_to_folder(note_id, folder_id)
    }

    /// 批量重排同一 folder 内笔记的 sort_order（自定义排序专用）
    pub fn reorder(db: &Database, ordered_ids: &[i64]) -> Result<(), AppError> {
        db.reorder_notes(ordered_ids)
    }

    /// 拖拽排序用：取当前筛选条件下**全部**笔记 id（不分页），按当前 sort_by 排序。
    /// 前端拿到完整 id 列表后做 arrayMove → reorder，保证跨页 sort_order 一致。
    pub fn list_ids_for_reorder(db: &Database, query: &NoteQuery) -> Result<Vec<i64>, AppError> {
        db.list_note_ids_for_reorder(
            query.folder_id,
            query.keyword.as_deref(),
            query.uncategorized.unwrap_or(false),
            query.include_descendants.unwrap_or(true),
            query.sort_by.as_deref(),
        )
    }

    /// 全部移到回收站（软删，可在回收站恢复）
    pub fn trash_all(db: &Database) -> Result<usize, AppError> {
        db.trash_all_notes()
    }

    /// 查询笔记列表（分页）
    pub fn list(db: &Database, query: &NoteQuery) -> Result<PageResult<Note>, AppError> {
        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(20).clamp(1, 100);

        let (items, total) = db.list_notes(
            query.folder_id,
            query.keyword.as_deref(),
            page,
            page_size,
            query.uncategorized.unwrap_or(false),
            // 默认递归子文件夹（符合用户直觉：点父目录看到所有子树笔记）
            query.include_descendants.unwrap_or(true),
            query.sort_by.as_deref(),
        )?;

        Ok(PageResult {
            items,
            total,
            page,
            page_size,
        })
    }

    // ─── T-003 隐藏笔记 ────────────────────────────

    /// 切换笔记"隐藏"状态
    pub fn set_hidden(db: &Database, id: i64, hidden: bool) -> Result<bool, AppError> {
        db.set_note_hidden(id, hidden)
    }

    /// 列出所有隐藏笔记（分页 + 可选目录过滤）
    pub fn list_hidden(
        db: &Database,
        page: Option<usize>,
        page_size: Option<usize>,
        folder_id: Option<i64>,
        uncategorized: Option<bool>,
    ) -> Result<PageResult<Note>, AppError> {
        let page = page.unwrap_or(1).max(1);
        let page_size = page_size.unwrap_or(20).clamp(1, 100);
        let (items, total) =
            db.list_hidden_notes(page, page_size, folder_id, uncategorized.unwrap_or(false))?;
        Ok(PageResult {
            items,
            total,
            page,
            page_size,
        })
    }

    /// 列出所有"含至少一篇隐藏笔记"的 folder_id（含 None=未分类）
    pub fn list_hidden_folder_ids(db: &Database) -> Result<Vec<Option<i64>>, AppError> {
        db.list_hidden_folder_ids()
    }

    /// 列出"临时编辑"笔记（以临时方式打开的外部 .md）—— 用于「临时文件」面板
    pub fn list_scratch(
        db: &Database,
        page: Option<usize>,
        page_size: Option<usize>,
    ) -> Result<PageResult<Note>, AppError> {
        let page = page.unwrap_or(1).max(1);
        let page_size = page_size.unwrap_or(20).clamp(1, 100);
        let (items, total) = db.list_scratch_notes(page, page_size)?;
        Ok(PageResult {
            items,
            total,
            page,
            page_size,
        })
    }

    /// 设置 / 取消"临时编辑"标记（取消 = 转为正式笔记，回到主列表与搜索）
    pub fn set_scratch(db: &Database, id: i64, scratch: bool) -> Result<(), AppError> {
        db.set_note_scratch(id, scratch)
    }

}
