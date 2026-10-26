//! 「快速记一笔」独立悬浮窗。
//!
//! 原实现位于 `services::popout_window`（笔记弹窗 / 思维导图弹窗 / 快速记录三者合一），
//! 本次删减去掉了前两者，只保留快速记录——它是快捷捕获功能的核心，属于保留清单。
//! 窗口加载前端 `#/quick-add` 路由（pages/quick-add 原样继承）。

use tauri::{AppHandle, Manager, WindowBuilder, WindowUrl};

use crate::error::AppError;

/// 「快速记一笔」悬浮窗 toggle 语义：
/// - 窗口不存在 → 创建（无边框 / 置顶 / 不进任务栏 / 居中小窗），加载 `#/quick-add`
/// - 已存在且可见 → 收起（hide，不销毁，下次秒开）
/// - 已存在但隐藏 → 前置并聚焦
///
/// 由全局快捷键（`global.quickAddWindow`）或托盘菜单调起；应用在后台也能唤起，不打断主窗工作。
/// 窗口内 Enter 追加到今日日记、Esc/失焦自动隐藏（前端处理）。
pub fn open_quick_add(app: &AppHandle) -> Result<(), AppError> {
    let label = "quick-add";

    if let Some(win) = app.get_window(label) {
        if win.is_visible().unwrap_or(false) {
            let _ = win.hide();
        } else {
            let _ = win.unminimize();
            let _ = win.show();
            let _ = win.set_focus();
        }
        return Ok(());
    }

    log::info!("[quick-add] 创建快速记一笔悬浮窗");

    WindowBuilder::new(app, label, WindowUrl::App("index.html#/quick-add".into()))
        .title("快速记一笔")
        .inner_size(560.0, 240.0)
        .center()
        .resizable(false)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        // 先隐藏建窗，规避新 WebView 加载期间的空白闪屏：
        // 前端 quick-add 页 React 首帧画好后再自己 show() + setFocus()
        .focused(false)
        .visible(false)
        .build()
        .map_err(|e| AppError::Custom(format!("快速记一笔窗口创建失败: {}", e)))?;

    Ok(())
}
