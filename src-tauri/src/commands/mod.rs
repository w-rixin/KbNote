pub mod attachment;
pub mod config;
pub mod daily;
// Excel 二维数据集（仅桌面端，模块内已 #![cfg(desktop)]）
// Excel 导入为笔记（仅桌面端，模块内已 #![cfg(desktop)]）
pub mod data_dir;
pub mod export;
// 系统字体枚举仅桌面端可用（font-enumeration 无 Android 后端）
#[cfg(desktop)]
pub mod folders;
pub mod image;
pub mod import;
// 移动端"检查更新"（桌面端用 tauri-plugin-updater，不需要这个）
#[cfg(mobile)]
pub mod notes;
#[cfg(desktop)]
// 待办提醒自定义提示音（导入 / 列举 / 删除音频文件）
pub mod snapshot;
pub mod search;
// 全局快捷键仅桌面端可用（移动端无此概念）
#[cfg(desktop)]
pub mod shortcut;
pub mod system;
pub mod tags;
pub mod tasks;
pub mod trash;
pub mod window;
