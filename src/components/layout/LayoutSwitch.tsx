import { ImportStatusDock } from "@/components/ui/ImportStatusDock";
import { AppLayout } from "./AppLayout";

/**
 * 路由根布局。
 *
 * 原 kb 这里是 LayoutSwitch（按 isMobile 在 AppLayout / MobileLayout 之间切换）——
 * 移动端已随删减移除，KbNote 是纯桌面应用，直接固定 AppLayout。
 * 批量导入悬浮条挂在这一层（而不是各页面内），进度才不会因为切页面而丢。
 */
export function LayoutSwitch() {
  return (
    <>
      <AppLayout />
      <ImportStatusDock />
    </>
  );
}
