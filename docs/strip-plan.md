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
| P3 | 联通：麒麟 deb（ABI 硬闸）+ Windows nsis | 两个打包工作流绿 |
| P4 | 打 tag v1.0.0，发布 Release（exe + deb） | Release 页面资产齐全 |

## 六、执行记录

### P0 基座（`f43b59a`，627 文件）
以 knowledge-base 1.65.0 的 tauri-v1 分支整树复制，改名映射见第四节，
force-push 重置远程 main。前端 `tsc --noEmit` 零错误 + `vite build` 通过。

### P1 后端删减（services 68 → 33，commands 40 → 17）
分两批，每批 CI 验证：
- **P1a**：AI / MCP / 推送簇 + `kb-core/`、`mcp/` 两个子 crate。
- **P1b**：知识组织（links/whiteboard/cards）、同步、提醒、多媒体、数据表、加密空间。

踩到并修掉的坑（都属于"删完没清干净"）：
- `sed` 删 `pub mod` 留下孤立 `#[cfg(mobile)]`，桌面构建下 `commands::notes` 整个消失
  → 完整重写 `commands/mod.rs` 与 `services/mod.rs`。
- 删 `rmcp` 后 tokio 的 `select!` / `#[tokio::test]` 宏消失 → 补 `macros,time` feature。
- `updater` feature 与 `tauri.conf.json` 的 allowlist 不匹配 → conf 块与 Cargo feature **同删**。
- `bundle.resources`（pdfium/ocr）随砍单删除后，tauri-build 校验 glob 缺失直接 exit 1
  → 一并删掉 `bundle.resources`。

**迁移链的处置（重要决策）**：`database/schema.rs` 的 64 步迁移链**原样保留，一步未剪**。
链条一旦断开，v0–v64 之间的任何老库都升不上来；代价是新库首次启动会把 64 步全跑一遍，
并保留 ai_models / cards / whiteboards 等已无功能对应的空表。
不清理的理由与风险已在 `kylin-v1-check.yml` 的 cargo test 步骤注释里写明。

### P2 前端删减（删 78 个文件）
AI、白板、图谱、卡片、幻灯片、同步、推送、移动端、更新器、应用锁、保险箱相关页面与组件；
`Router.tsx` 重写为桌面单栏（HashRouter + 固定 AppLayout）；
`MicButton` 降级为 `return null` 桩（保留调用点形状）；
`NewTodoButton` 去掉 AI 规划下拉。

随后又清掉 P2 的漏网之鱼：`MobileHome` / `MobileMe` / `MobileSearch` / `MobileTags`
四个移动端页面，以及 4 处 `useIsMobile` Wrapper。

### 品牌统一（26 处 / 13 文件）
只改**用户可见的品牌串**：崩溃报告头与原生弹窗标题、升级提示、诊断包文件名、
导出根目录名（`知识库导出_` → `KbNote导出_`）、关于页。
作为普通名词的「知识库」（"个人知识库""知识库工程化"）**保留**——那是自然中文，改了反而别扭。

### P3 联通（实测）
麒麟 aarch64 deb 构建成功，产物与 ABI 实测：

```
Package: kb-note            Architecture: arm64
Depends: libc6, librsvg2-2, fonts-noto-cjk,
         libappindicator3-1, libwebkit2gtk-4.0-37, libgtk-3-0
最高 GLIBC: GLIBC_2.30       （≤ 2.31，麒麟 V10 GFB 的 focal 底满足）
DT_NEEDED: libssl.so.1.1 / libcrypto.so.1.1  （OpenSSL 1.1.1，不是 3）
           libwebkit2gtk-4.0.so.37           （4.0，不是 4.1）
```

`libssl.so.1.1` 这一项是 kb run 9 事故（麒麟上一启动就崩）的正面验证。

### `cargo test` 首次跑测：467 通过 / 3 失败
加了 `cargo test --lib` 之后第一次跑就抓出问题，三类各一：

1. **精简引入的真 bug**：`legacy_provider_key` 少映射了 `claude` / `hunyuan`，
   落进 `_ => "custom"`；kb 原实现归到 `openai_compatible_custom`（语义不同）。
   已补回——迁移链对 KbNote 无功能意义，但只要老库要升级，行为就必须与 kb 逐字一致。
2. **平台前提不成立**（两例）：用例用 Windows **反斜杠**路径，而 Linux 上 `\`
   是合法文件名字符，`C:\Users\...` 整个串被 `Path` 视作单个文件名，恒扫不到资产段。
   处理：反斜杠版本限定 `#[cfg(windows)]`，各补一条 POSIX / 正斜杠等价用例保住 Linux CI 的覆盖。

## 七、明确不做（v1 范围外）

双向链接图谱、白板、卡片背诵、思维导图、AI 问答、多端同步（留 v2.1）、
移动端、PDF 导出、笔记模板、任务提醒推送、加密保险库。
