//! 任务分组（项目）的 IPC 入口。
//!
//! 🔴 同一类漏法的第三例（前两例见 `commands/links.rs`、`commands/converter.rs`）：
//! `database/projects.rs` 完整保留（含归档、稳定 UUID、供同步用的变体），
//! P1b 只删掉了命令层 —— 于是新建待办弹窗里的「项目」下拉、项目管理弹窗、
//! 甘特图的项目筛选全部 command not found。
//!
//! 为什么是补而不是删：项目是任务模块的分组维度（v41 起），
//! `CreateTaskModal` / `TaskDetailModal` / `GanttView` / `DataviewBlockModal`
//! 四处都依赖它。删掉等于把"按案件 / 主题分组待办"这个能力一起砍了，
//! 代价远大于补五个薄命令。

use crate::models::{CreateProjectInput, Project, UpdateProjectInput};
use crate::state::AppState;

/// 列出项目。默认不含已归档（用户给归档项目挂新任务的场景极少）。
#[tauri::command]
pub fn list_projects(
    state: tauri::State<'_, AppState>,
    include_archived: bool,
) -> Result<Vec<Project>, String> {
    state
        .db
        .list_projects(include_archived)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_project(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<Option<Project>, String> {
    state.db.get_project(id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_project(
    state: tauri::State<'_, AppState>,
    input: CreateProjectInput,
) -> Result<i64, String> {
    state.db.create_project(input).map_err(|e| e.to_string())
}

/// 更新项目（改名 / 改色 / 归档 / 起止日期）。
/// 返回 bool 表示是否真的改到了行；前端 invoke<void> 会忽略这个值。
#[tauri::command]
pub fn update_project(
    state: tauri::State<'_, AppState>,
    id: i64,
    input: UpdateProjectInput,
) -> Result<bool, String> {
    state
        .db
        .update_project(id, input)
        .map_err(|e| e.to_string())
}

/// 删除项目。任务的 `project_id` 因 `ON DELETE SET NULL` 自动落到"无项目"，
/// 不会连带删任务。
#[tauri::command]
pub fn delete_project(
    state: tauri::State<'_, AppState>,
    id: i64,
) -> Result<bool, String> {
    state.db.delete_project(id).map_err(|e| e.to_string())
}
