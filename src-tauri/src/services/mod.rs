pub mod asset_path;
// AI 回答的引用标记解析 + 白名单校验（模型自报的引用编号不可直接采信）
pub mod attachment;
pub mod config;
pub mod converter;
// Excel 二维数据集识别：区域切分 + 表头启发式 + 字段画像（全确定性规则，不用 LLM）
// 纯算法、无 calamine 依赖，故双端可编
// 数据集入库编排：解析 → 识别 → 落库。依赖 excel_parser，仅桌面端
pub mod daily;
pub mod data_dir;
pub mod db_recovery;
// 紧急待办全屏窗口仅桌面端（移动端无多窗口概念）
#[cfg(desktop)]
// Excel 解析仅桌面端：calamine 在 Android target 编译失败
// （T-M013 移动端导入改单文件模式后再决定是否引入）
#[cfg(desktop)]
// Excel 导入为笔记：解析 + 渲染 + 落盘源文件，依赖 excel_parser，仅桌面端
#[cfg(desktop)]
pub mod export;
pub mod export_html;
pub mod export_merge;
// Word 导出仅桌面端：docx_rs 在 Android target 编译失败
#[cfg(desktop)]
pub mod export_word;
pub mod folder;
pub mod hash;
pub mod http_client;
pub mod image;
pub mod image_download;
pub mod import;
pub mod legacy_api_url;
pub mod import_attachments;
pub mod markdown;
// 外部 MCP server 子进程管理仅桌面端：rmcp transport-child-process 在桌面端 dependencies
// 移动端 fork/spawn 受限，砍掉外部 MCP，仅保留 in-memory 内置 server（kb-core）
#[cfg(desktop)]
// MCP HTTP 服务：把自家知识库暴露给外部 agent。仅桌面端
// （移动端没有"被别的进程连"的场景，也不该在手机上开监听端口）
#[cfg(desktop)]
pub mod note;
pub mod note_excerpt;
#[cfg(desktop)]
// macOS 零二进制 OCR（Apple Vision FFI）；仅 macOS 编译
#[cfg(target_os = "macos")]
// 笔记 pop-out 窗口仅桌面端（移动端改 Modal）
#[cfg(desktop)]
pub mod plugin;
// 待办提醒自定义提示音文件管理（复制进 app_data_dir 以落进 assetProtocol scope）
pub mod quick_capture;
pub mod quick_window;
pub mod safe_filename;
pub mod search;
// 全局快捷键仅桌面端可用
#[cfg(desktop)]
pub mod shortcut;
pub mod snapshot;
pub mod tag;
pub mod tasks;
pub mod trash;
// 出网 URL 的 SSRF 校验，仅供剪藏 / 外链图片用（AI / WebDAV / S3 走用户自填地址，不适用）
pub mod url_safety;
pub mod window_size;
// Windows 消息过滤：拦掉 Alt 触发的系统菜单模态循环（无边框窗口白屏卡死根因）
#[cfg(target_os = "windows")]
pub mod win_msg_guard;
