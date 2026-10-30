//! 笔记双链（`[[标题]]`）的 IPC 入口。
//!
//! 🔴 为什么会有这个文件（删减时最容易漏的一类）：
//! P1b 把 `services/links.rs` 和 `commands/links.rs` 一起删了，但**数据层
//! `database/links.rs` 完整留着**（881 行：`note_links` 表读写、标题规范化匹配、
//! 反链汇总、`[[Title|ID]]` 显式锚点解析）。结果就是前端 wiki 链接的自动补全、
//! `[[X]]` 跳转、编辑器底部的链接状态条全部调不到命令 —— 属于"后端还在、只差一层门"。
//!
//! 本文件**不新增任何业务逻辑**，逐个转调数据层，把门补回来。
//!
//! 为什么不干脆把 `database/links.rs` 也删掉：`[[X]]` 双链是编辑器核心能力
//! （保留清单里"笔记核心"的一部分），`note_links` 表也还在迁移链里，
//! 删掉等于把已修好的功能再砍一次。

use crate::models::{NoteLink, NoteLinkSummary, WikiLinkSuggestItem};
use crate::state::AppState;

/// 同步某篇笔记的出链（先清旧、再插新）。前端 handleSave 时调用。
#[tauri::command]
pub fn sync_note_links(
    state: tauri::State<'_, AppState>,
    source_id: i64,
    target_ids: Vec<i64>,
) -> Result<(), String> {
    state
        .db
        .sync_note_links(source_id, target_ids)
        .map_err(|e| e.to_string())
}

/// 反链列表（谁引用了我）。
#[tauri::command]
pub fn get_backlinks(
    state: tauri::State<'_, AppState>,
    note_id: i64,
) -> Result<Vec<NoteLink>, String> {
    state
        .db
        .get_backlinks(note_id)
        .map_err(|e| e.to_string())
}

/// 链出 / 链入 / 断链一次取回（编辑器底部链接状态条用，省两次 IPC）。
#[tauri::command]
pub fn get_note_link_summary(
    state: tauri::State<'_, AppState>,
    note_id: i64,
) -> Result<NoteLinkSummary, String> {
    state
        .db
        .get_note_link_summary(note_id)
        .map_err(|e| e.to_string())
}

/// 从正文重新解析 `[[wiki]]` 并同步出链（日记页保存后调 ——
/// 日记此前从不同步出链，写进去的 `[[X]]` 不进 `note_links`）。
#[tauri::command]
pub fn rebuild_note_links(
    state: tauri::State<'_, AppState>,
    note_id: i64,
    content: String,
) -> Result<(), String> {
    state
        .db
        .rebuild_note_links_from_content(note_id, &content)
        .map_err(|e| e.to_string())
}

/// `[[` 自动补全的候选（按标题模糊匹配）。
///
/// 数据层已过滤隐藏 / 临时 / 已删除笔记 —— 候选下拉不暴露隐藏标题（弱保护）；
/// 但用户已经写好的 `[[隐藏笔记]]` 仍能跳转（走 `find_note_id_by_title_loose`，
/// 那条路不过滤）。
#[tauri::command]
pub fn search_link_targets(
    state: tauri::State<'_, AppState>,
    keyword: String,
    limit: Option<usize>,
) -> Result<Vec<WikiLinkSuggestItem>, String> {
    state
        .db
        .search_notes_by_title(&keyword, limit.unwrap_or(20))
        .map_err(|e| e.to_string())
}

/// `[[标题]]` 的精确跳转（trim + 空白折叠 + 大小写不敏感）。
#[tauri::command]
pub fn find_note_id_by_title_loose(
    state: tauri::State<'_, AppState>,
    title: String,
) -> Result<Option<i64>, String> {
    state
        .db
        .find_note_id_by_title_loose(&title)
        .map_err(|e| e.to_string())
}
