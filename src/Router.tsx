import { createHashRouter, RouterProvider } from "react-router-dom";
import { LayoutSwitch } from "@/components/layout/LayoutSwitch";
import { RouteErrorFallback } from "@/components/ui/ErrorBoundary";
import HomePage from "@/pages/home";
import NoteListPage from "@/pages/notes";
import NoteEditorPage from "@/pages/notes/editor";
import SearchPage from "@/pages/search";
import TagsPage from "@/pages/tags";
import TrashPage from "@/pages/trash";
import DailyPage from "@/pages/daily";
import TasksPage from "@/pages/tasks";
import SettingsPage from "@/pages/settings";
import AboutPage from "@/pages/about";
import QuickCapturePage from "@/pages/quick-capture";
import QuickAddPage from "@/pages/quick-add";

/**
 * 路由表（KbNote 删减版）。
 *
 * 原 kb 的路由还包含：AI 对话、白板、知识图谱、卡片背诵、提示词、推送、
 * 隐藏空间、移动端全套、迁移 splash、紧急提醒接管窗、思维导图弹窗 —— 均已随功能删除。
 *
 * 独立路由（不挂 AppLayout）：
 * - /quick-capture 沉浸式闪念捕获页
 * - /quick-add     「快速记一笔」悬浮窗（quick_window.rs 创建窗口时加载）
 */
const router = createHashRouter([
  {
    path: "/quick-capture",
    element: <QuickCapturePage />,
    errorElement: <RouteErrorFallback />,
  },
  {
    path: "/quick-add",
    element: <QuickAddPage />,
    errorElement: <RouteErrorFallback />,
  },
  {
    path: "/",
    element: <LayoutSwitch />,
    errorElement: <RouteErrorFallback />,
    children: [
      { index: true, element: <HomePage /> },
      { path: "notes", element: <NoteListPage /> },
      { path: "notes/:id", element: <NoteEditorPage /> },
      { path: "search", element: <SearchPage /> },
      { path: "tags", element: <TagsPage /> },
      { path: "trash", element: <TrashPage /> },
      { path: "daily", element: <DailyPage /> },
      { path: "tasks", element: <TasksPage /> },
      { path: "settings", element: <SettingsPage /> },
      { path: "about", element: <AboutPage /> },
    ],
  },
]);

export function AppRouter() {
  return <RouterProvider router={router} />;
}
