# KbNote 删减计划（从 knowledge-base 1.65.0 到 KbNote v1.0.0）

> 本文件是项目的唯一删减依据。任何模块的去留以此为准，改动需更新本文件。

## 一、原则

1. **基座不动**：以 knowledge-base 1.65.0（tauri-v1 分支）整树复制为基座。
   保留清单内的代码**原样继承**，不重写、不"优化"。
2. **只做减法**：删模块、删页面、删依赖、删配置。命名统一改（见改名映射）。
3. **小步验证**：后端每删一批跑 `cargo check`（CI focal 容器），前端每删一批跑 `tsc`。
4. **数据不迁移**：沿继承数据库 schema 与迁移链；遗留表清理用新增迁移（v65 drop），
   不篡改历史迁移。

## 二、保留清单（原样继承）

### 编辑器（核心体验，绝不重写）
- `components/editor/TiptapEditor.tsx` — tiptap 3 富文本编辑器
- `components/editor/EditorOutline.tsx` — **大纲面板**
- `components/editor/EditorToolbar.tsx` — 工具栏
- `components/editor/CodeBlockEnhanced.tsx` — 代码块增强（语言选择/复制）
- `components/editor/MarkdownSourceEditor.tsx` — Markdown 源码模式切换
- `headingAnchor.ts` / `HeadingNumber.ts` — 标题锚点与编号
- `EditorStats.tsx` — 字数统计
- Callout / Columns / ImageResize / Annotation 等节点扩展（按编译依赖保留）

### 笔记核心
| 后端 | 说明 |
|---|---|
| `services/note.rs` | 笔记 CRUD/软删除/置顶 |
| `services/folder*.rs` | 文件夹 |
| `services/tag*.rs` | 标签 |
| `services/search.rs` + `cjk.rs` + `fusion.rs` | FTS5 + 中文融合搜索 |
| `services/snapshot.rs` | 版本快照 |
| `services/attachment*.rs` / `image*.rs` | 附件与图片 |
| `services/export*.rs` | 导出 md/html/**docx** |
| `services/import*.rs` | 导入 md/txt（编码探测） |
| `services/daily.rs` / `tasks.rs` | 每日 / 任务 |
| `services/quick_capture.rs` | 快捷捕获 |
| `services/config.rs` / `system.rs` / `data_dir.rs` / `db_recovery.rs` | 设置/系统/数据目录/坏库恢复 |
| `services/window*.rs` / `shortcut.rs` / `win_msg_guard.rs` | 窗口/快捷键/无边框守卫 |

## 三、砍掉清单

| 类别 | 模块/文件 | 关联删除 |
|---|---|---|
| AI 全家桶 | `services/ai.rs` `anthropic.rs` `model_service.rs` `mcp_client.rs` `mcp_http.rs` `skills.rs` `prompts.rs` `citations.rs`；`commands/ai.rs` `mcp.rs` `skills.rs` `prompts.rs` `model*.rs` | 子 crate `kb-core/` `mcp/`、`rmcp` 依赖、`externalBin` kb-mcp、`build:mcp` 脚本、State 的 mcp 字段 |
| 知识组织（v2+ 再议） | `links.rs`（双向链接）、`graph` 页面、`whiteboard`、`cards`（fsrs）、`mindmap` | 前端 @antv/g6、ts-fsrs、excalidraw |
| 同步 | `sync*.rs`、`sync_v1/`、`commands/sync*.rs` | 前端 sync 页面、webdav |
| 提醒/推送 | `push*.rs`、`reminder_sound.rs`、`task_reminder.rs`、`emergency_window.rs` | 前端 push/push-popup/emergency-reminder 页面 |
| 多媒体 | `asr*`、`ocr*`、`pdf*`、`videos*` | `resources/ocr`、`resources/pdfium`、pdfium-render、pdf-extract |
| 数据表 | `dataset*`、`dataview*`、`excel_import*` | calamine、前端 dataview 节点 |
| 加密空间 | `vault.rs`、`hidden*`、`app_lock*` | 前端 hidden/applock 页面 |
| 其他 | `projects.rs`（项目）、`web_clip.rs`（网页剪藏）、`source_file.rs`/`source_writeback.rs`、`daily_import.rs`、`template.rs`（模板，v2 再议）、`orphan*.rs`、`url_mapping.rs`、`legacy_api_url.rs` | 对应页面与命令 |
| 移动端 | `commands/mobile_update.rs`、`pages/notes/Mobile*.tsx`、mobile 分支逻辑 | `.github/workflows/android.yml`、`mobile` 相关 npm 脚本 |

## 四、改名映射（P0 已应用）

| 原 | 新 |
|---|---|
| crate/lib `knowledge_base` / `knowledge_base_lib` | `kbnote` / `kbnote_lib` |
| identifier `com.agilefr.kb` / `com.agilefr.kb-dev` | `com.agilefr.kbnote` / `com.agilefr.kbnote-dev` |
| 产品名 `Knowledge Base` / 窗口标题 `知识库` | `KbNote` |
| dev 端口 1421 | 1422 |
| 版本 1.65.0 | 1.0.0 |

## 五、阶段与验收

| 阶段 | 内容 | 验收 |
|---|---|---|
| P0 | 整树复制 + 改名 + 配置调整 + 仓库重置 | 前端 tsc 通过；CI frontend check 绿 |
| P1 | 后端删减（AI 簇→其余模块），state/lib/Cargo 瘦身 | focal 容器 `cargo check --all-targets` + `cargo test` 绿 |
| P2 | 前端删减（页面/组件/依赖/路由） | `tsc` + `vite build` 绿 |
| P3 | 联通：麒麟 deb（ABI 硬闸）+ Windows nsis/msi | 三个 CI 工作流全绿 |
| P4 | 打 tag v1.0.0，发布三平台 Release | Release 页面资产齐全 |

## 六、明确不做（v1 范围外）

双向链接图谱、白板、卡片背诵、思维导图、AI 问答、多端同步（留 v2.1）、
移动端、PDF 导出、笔记模板、任务提醒推送、加密保险库。
