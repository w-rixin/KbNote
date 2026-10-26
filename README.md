# KbNote

**本地单机笔记应用** —— 笔记 / 每日笔记 / 任务三件套，数据全部留在本机 SQLite，不联网、不上云。

KbNote 由 [knowledge-base](https://github.com/w-rixin/knowledge-base) 1.65.0 **功能删减**而来：
整树继承其 Tauri 1.x 架构、tiptap 3 编辑器（含大纲）、全文搜索与麒麟 V10 兼容能力，
移除此范围之外的功能模块（见 [docs/strip-plan.md](docs/strip-plan.md)）。

> 状态：删减完成，等待发 v1.0.0。保留清单内的实现（编辑器 / 大纲 / 搜索 / 快照 /
> 每日 / 任务）原样继承、未做重写；砍掉清单内的模块已分批移除完毕。
> **删减依据与执行记录见 [docs/strip-plan.md](docs/strip-plan.md)**（含各阶段实测数据：
> 后端 services 68→33、commands 40→17，前端删 78 个文件，麒麟 deb 的 ABI 实测结果）。

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

两条流水线分管两个目标平台，**产物合流到同一个 Release**：

| 平台 | 工作流 | 产物 |
|---|---|---|
| 麒麟 V10 / 飞腾 / 鲲鹏（aarch64） | `kylin-deb.yml` | `kb-note_1.0.0_arm64.deb` |
| Windows | `release.yml` | NSIS 安装包（.exe） |

macOS 不在 v1 范围内（无目标机，构建矩阵已移除）。

### 麒麟 deb 为什么只能走 `kylin-deb.yml`

它在 `ubuntu:22.04-arm` 机器上的 **ubuntu:20.04（focal）容器**里编译，与目标机 ABI 同源
（glibc 2.31 / OpenSSL 1.1.1），打包前还有硬闸校验：最高 GLIBC 符号 ≤ `GLIBC_2.31`、
DT_NEEDED 不得出现 `libssl.so.3`、`ldd` 不得有 `not found`。

🔴 不要改用 `release.yml` 里曾经有过的 aarch64 条目：那个条目在 jammy 宿主机上直接编译，
产出链接 glibc 2.35 + OpenSSL 3 的二进制，装到麒麟 **启动即崩**，而 `dpkg -i` 与
`apt -f install` 全绿（deb 的 Depends 不声明 OpenSSL，包管理器查不出来）。

### 发一个版本（顺序不能颠倒）

1. 跑 `release.yml`（手动触发填版本号，或推 `v1.0.0` tag）
   → 它的 prepare-release 先创建 draft Release，再构建 Windows 安装包并上传。
2. 跑 `kylin-deb.yml`，`release_tag` 填 `v1.0.0`
   → 构建麒麟 deb 并上传到**同一个** Release。
   （本工作流刻意不自建 Release：两条流水线并发 `gh release create` 会撞 GitHub API race。）
3. 检查两个资产都在，把 draft 发布。

## 许可：双许可

- **AGPL-3.0**：开源免费，衍生作品须同样以 AGPL 开源（见 [LICENSE](LICENSE)）
- **商业授权**：付费豁免 copyleft，允许闭源集成 / SaaS / OEM（见 [COMMERCIAL-LICENSE.md](COMMERCIAL-LICENSE.md)）

## 与 knowledge-base 的关系

KbNote 是独立产品、独立仓库（原仓库与 v1.65.0 数据不受影响）。
两者为同一代码血脉：KbNote 只做减法与改名，技术决策与踩坑记录见
[AGENTS.md](AGENTS.md)（继承自 knowledge-base 的工程约定）。
