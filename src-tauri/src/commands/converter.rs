//! Word（.doc / .docx）读取相关的 IPC 入口。
//!
//! 背景与 `commands/links.rs` 完全同源：`services/converter.rs`（333 行，
//! LibreOffice + Word ProgId 双路 `.doc → .docx` 转换、探测器、诊断报告）在删减时
//! **完整保留**，被删掉的只是命令层。于是 Word 导入（`lib/wordImport.ts`）和
//! `.docx` 附件预览（`components/preview/DocxPreview.tsx`）都调不到命令。
//!
//! 这里补回纯 IO / 纯转调的那几个；凡是依赖已删数据表的**不补**：
//! - `attach_source_file` / `get_source_file_absolute_path` 依赖 `source_files` 表
//!   与 source_file 服务（已整体删除），属于"笔记关联一个外部源文件"这个功能本身
//!   被砍掉，前端的调用点已同步降级，不在这里硬凑。
//!
//! 为什么值得补而不是连前端一起删：.docx 是附件预览的常见格式
//! （`AttachmentPreviewModal` 会路由到它），而且转换器代码一行没删，
//! 补四个薄命令就能让两条路径都活过来 —— 比删掉一个能用的功能划算。

use std::path::PathBuf;

use base64::Engine;
use tauri::State;

use crate::services::converter::{self, ConverterDiagnostic, DocConverter};
use crate::state::AppState;

/// 探测系统可用的 `.doc` 转换器（前端启动时查一次）。
#[tauri::command]
pub fn get_converter_status() -> DocConverter {
    converter::detect_converter()
}

/// 详细诊断：每个 Word ProgId 的实测结果（含 PowerShell 错误）。
///
/// 每次调用都会重新探测、不走缓存 —— 设置页里点"诊断"就是想让用户看到
/// 当前真实的可用性，缓存会让这个按钮失去意义。
#[tauri::command]
pub fn diagnose_doc_converter() -> ConverterDiagnostic {
    converter::diagnose()
}

/// 把任意本地文件读成 base64。
///
/// 路径来自前端 `dialog.open()` 的选择，所以不额外限制目录 —— 用户主动选的
/// 就是他授权访问的。真正需要防的是"前端拼出来的路径"，那类调用不走这个命令。
#[tauri::command]
pub fn read_file_as_base64(path: String) -> Result<String, String> {
    let bytes = std::fs::read(&path).map_err(|e| format!("读取文件失败 {path}: {e}"))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
}

/// 把 `.doc` 转成 `.docx` 后返回 base64（前端喂给 mammoth 渲染）。
///
/// 输出落在 `<data_dir>/converter-tmp/`，而不是源文件旁边：源文件可能位于
/// 只读位置（光盘、共享盘）或用户不希望被写脏的目录。
#[tauri::command]
pub fn convert_doc_to_docx_base64(
    state: State<'_, AppState>,
    path: String,
) -> Result<String, String> {
    let src = PathBuf::from(&path);
    let dst_dir = state.data_dir.join("converter-tmp");
    let out = converter::convert_doc_to_docx(&src, &dst_dir).map_err(|e| e.to_string())?;
    let bytes = std::fs::read(&out).map_err(|e| format!("读取转换结果失败: {e}"))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
}
