//! Service 层：业务逻辑集中地。
//!
//! KbNote 由 knowledge-base 1.65.0 删减而来，本文件只列**保留清单**内的模块
//! （AI/MCP/白板/卡片/图谱/同步/提醒/多媒体/加密库等已随删减移除，
//! 清单见 docs/strip-plan.md）。
//!
//! 平台差异：本项目是纯桌面应用（Tauri 1.x 无移动端），原先的
//! `#[cfg(desktop)]` / `#[cfg(mobile)]` 门控一并去掉，避免门控与模块列表漂移。

pub mod asset_path;
pub mod attachment;
pub mod config;
pub mod converter;
pub mod daily;
pub mod data_dir;
pub mod db_recovery;
pub mod export;
pub mod export_html;
pub mod export_merge;
pub mod export_word;
pub mod folder;
pub mod hash;
pub mod http_client;
pub mod image;
pub mod image_download;
pub mod import;
pub mod import_attachments;
pub mod legacy_api_url;
pub mod markdown;
pub mod note;
pub mod note_excerpt;
pub mod quick_capture;
pub mod quick_window;
pub mod safe_filename;
pub mod search;
pub mod shortcut;
pub mod snapshot;
pub mod tag;
pub mod tasks;
pub mod trash;
pub mod url_safety;
pub mod window_size;
/// Windows 消息过滤：拦掉 Alt 触发的系统菜单模态循环（无边框窗口白屏卡死根因）。
/// 仅 Windows 编译（lib.rs 的调用点也有对应 cfg）。
#[cfg(target_os = "windows")]
pub mod win_msg_guard;
