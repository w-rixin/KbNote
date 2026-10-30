import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import path from "path";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

/**
 * KbNote 构建配置。
 *
 * 与 knowledge-base 原版的差异（砍掉白板之后）：
 *
 * 1. **删掉 `kb-excalidraw-assets` 插件** —— 它原本在每次 buildStart 时把
 *    `node_modules/@excalidraw/excalidraw/dist/prod/fonts`（234 个文件 / 14MB）
 *    同步进 `public/excalidraw/fonts`，让离线环境下的白板手写体可用。
 *    白板已删，这 14MB 却会原样打进 deb / exe —— 是安装包里最大的一块纯浪费。
 *    （注意：只删 public 目录没用，插件下次构建又会把它灌回来。）
 * 2. 删掉 `process.env.IS_PREACT` 注入与 `vendor-excalidraw` 分块，原因同上。
 * 3. **dev 端口 1421 → 1422**：原版用的是 kb 的 1421，而 KbNote 的
 *    `tauri.conf.json#build.devPath` 与 `package.json#dev:clean`（kill-port）都已是 1422。
 *    三者不一致时 `pnpm tauri:dev` 会连不上 dev server（vite 起在 1421、
 *    Tauri 去连 1422），表现为 dev 模式白屏 —— 是 P0 改名时漏掉的一处。
 */
export default defineConfig(async () => ({
  plugins: [react(), tailwindcss()],

  resolve: {
    alias: {
      "@": path.resolve(__dirname, "src"),
    },
  },

  // 依赖预打包目标 es2022：@tiptap 3 / antd 6 的产物用了较新语法，
  // 交给 esbuild 按这个目标预打包；生产构建再由下面的 build.target 统一降级
  //（这两件事互不冲突：前者只影响 dev/optimizeDeps）。
  optimizeDeps: {
    esbuildOptions: { target: "es2022" },
  },

  build: {
    // 生产构建优化
    // ⚠️ macOS 12 Monterey 自带 WKWebView = Safari 15.6，不支持 ES2023
    // (Array.findLast / toSorted / toReversed 等)。esnext 不做任何转译，
    // 一旦项目依赖（tiptap 3 / antd 6 / lucide 等）输出 ES2023 语法就会
    // 抛 SyntaxError 让整个 chunk 加载失败 → 白屏。
    // 把 target 限到 safari15 让 esbuild 把 ES2023 降级到 ES2020 兼容代码。
    // chrome88/edge88/firefox88 是 antd 6 官方最低门槛，对齐避免漏网。
    //
    // 对 KbNote 更要紧的是麒麟侧的 WebKitGTK 4.0（老 JSC）：ES2018 的 lookbehind
    // 那条限制不归这里管，由 scripts/apply-es2018-patches.mjs 在 postinstall 处理。
    target: ["es2020", "safari15", "chrome88", "edge88", "firefox88"],
    minify: "terser",
    terserOptions: {
      compress: { drop_console: true, drop_debugger: true },
    },
    rollupOptions: {
      output: {
        manualChunks: {
          "vendor-react": ["react", "react-dom", "react-router-dom"],
          "vendor-antd": ["antd", "@ant-design/icons"],
          "vendor-editor": ["@tiptap/react", "@tiptap/starter-kit"],
        },
      },
    },
    chunkSizeWarningLimit: 1000,
  },

  clearScreen: false,
  server: {
    port: 1422,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          // 与 1422 同段位（原版 1421/1431 的下一对）
          port: 1432,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
