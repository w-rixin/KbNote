//! Command 层：Tauri IPC 入口，薄封装，业务逻辑都在 services/。
//!
//! 模块列表与 services 的保留清单一一对应；平台门控已随移动端移除一并去掉。

pub mod attachment;
pub mod config;
pub mod daily;
pub mod data_dir;
pub mod export;
pub mod folders;
pub mod image;
pub mod import;
pub mod notes;
pub mod search;
pub mod shortcut;
pub mod snapshot;
pub mod system;
pub mod tags;
pub mod tasks;
pub mod trash;
pub mod window;
