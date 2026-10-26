# KbNote

**本地单机笔记应用** —— 笔记 / 每日笔记 / 任务三件套，数据全部留在本机 SQLite，不联网、不上云。

KbNote 由 [knowledge-base](https://github.com/w-rixin/knowledge-base) 1.65.0 **功能删减**而来：
整树继承其 Tauri 1.x 架构、tiptap 3 编辑器（含大纲）、全文搜索与麒麟 V10 兼容能力，
移除此范围之外的功能模块（见 [docs/strip-plan.md](docs/strip-plan.md)）。

> 状态：删减进行中。保留清单内的实现（编辑器/大纲/搜索/快照/每日/任务）原样继承，
> 不做重写；砍掉清单内的模块正在分批移除。

## 功能（目标范围）

| 模块 | 内容 |
|---|---|
| 编辑器 | tiptap 3 富文本 Markdown：工具栏 / **大纲导航** / 代码高亮 / 表格 / 任务列表 / 图片 / 源码模式切换 / 字数统计 |
| 笔记 | 文件夹树（多级）/ 标签 / 置顶 / 回收站（软删除）/ 自动保存 |
| 版本 | 编辑自动快照 + 手动保存版本 + 一键恢复（恢复前留安全快照） |
| 搜索 | 全文搜索：FTS5 + 中文 CJK 融合（子串可搜、大小写不敏感） |
| 每日 | 一天一篇，日期导航 / 当月记录 / 心情 / 天气 |
| 任务 | 待办 / 进行中 / 已完成，优先级 / 截止日期 / 逾期标红 |
| 导入导出 | 导入 md / txt（编码探测）；导出 md / html / docx |
| 快捷捕获 | 全局快捷键唤起无边框小窗，首行即标题 |
| 其他 | 亮暗主题、无边框窗口、设置页、数据目录管理 |

## 技术栈

- **桌面框架**：Tauri 1.x（Rust 后端 + WebView 前端），为麒麟 V10（glibc 2.31）而选
- **前端**：React 19 + TypeScript + antd 6 + TailwindCSS 4 + Zustand + React Router 7（HashRouter）+ Vite 7 + tiptap 3
- **后端**：Rust 2021 + rusqlite（SQLite bundled，FTS5）、serde、thiserror
- **架构**：Commands（IPC 入口）→ Services（业务逻辑）→ Database（数据访问）

## 开发

```bash
pnpm install
pnpm tauri:dev      # 开发模式（前端 1422 端口）
pnpm build          # tsc + vite build
cd src-tauri && cargo test
```

## 构建

| 平台 | 方式 |
|---|---|
| 麒麟 V10 / 飞腾 / 鲲鹏（aarch64） | 推 `kylin-deb-*` tag → CI 在 ubuntu:20.04 容器构建 deb（glibc 2.31 ABI 硬闸） |
| Windows | 推 `v*` tag → CI 构建 NSIS/MSI 并创建 Release |
| macOS | `pnpm tauri build`（最低 10.15） |

## 许可：双许可

- **AGPL-3.0**：开源免费，衍生作品须同样以 AGPL 开源（见 [LICENSE](LICENSE)）
- **商业授权**：付费豁免 copyleft，允许闭源集成 / SaaS / OEM（见 [COMMERCIAL-LICENSE.md](COMMERCIAL-LICENSE.md)）

## 与 knowledge-base 的关系

KbNote 是独立产品、独立仓库（原仓库与 v1.65.0 数据不受影响）。
两者为同一代码血脉：KbNote 只做减法与改名，技术决策与踩坑记录见
[AGENTS.md](AGENTS.md)（继承自 knowledge-base 的工程约定）。
