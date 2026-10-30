import { useEffect, useState } from "react";
import { Modal, Button, App as AntdApp, List } from "antd";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { exit } from "@tauri-apps/api/process";
import { useTabsStore, type NoteTab } from "@/store/tabs";
import { noteApi } from "@/lib/api";

/**
 * 监听托盘"退出"事件 → 检查未保存草稿 → 三选一确认 → 退出。
 *
 * 流程：
 *   - 无 dirty tab：直接走 syncThenExit()（现在就是 exit(0)）
 *   - 有 dirty tab：弹 Modal，让用户选择
 *     - 保存并退出：循环 dirty tabs，从 store draft 取内容 → noteApi.update → 退出
 *     - 放弃修改并退出：直接退出
 *     - 取消：关闭 Modal，什么都不做
 *
 * 注：托盘 quit 菜单项不再直接 app.exit(0)，而是 emit "tray:request-exit"，由本组件接管。
 * 注：原版这里是"隐藏到托盘 → 后台 push 所有同步后端 → 成功后 exit"，
 *     同步功能删减后 push 必然失败，那条链已移除，详见 syncThenExit 的注释。
 */
export function ExitConfirmListener() {
  const { message } = AntdApp.useApp();
  const [dirtyTabs, setDirtyTabs] = useState<NoteTab[]>([]);
  const [exiting, setExiting] = useState(false);
  const open = dirtyTabs.length > 0;

  useEffect(() => {
    let unlisten: UnlistenFn | null = null;
    listen("tray:request-exit", async () => {
      const tabs = useTabsStore.getState().getDirtyTabs();
      if (tabs.length === 0) {
        // 没有未保存草稿：仍然要尝试推送（DB 里可能有上次同步后的修改）
        await syncThenExit();
        return;
      }
      setDirtyTabs(tabs);
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      unlisten?.();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  /**
   * 退出入口。
   *
   * 原版：hide() 到托盘 → 后台把所有同步后端 push 一遍 → 全部成功才 exit，
   * 失败则把窗口重新显示出来弹「同步失败」，让用户选强制退出或留下处理。
   *
   * 同步功能（sync v1）已随删减移除，`syncV1Api.listBackends()` 必然 reject，
   * 于是"失败"分支每次都会命中 —— 用户每次点托盘退出都会被一个
   * 「读取后端列表失败」的框拦一下，还得再点一次"强制退出"。
   * 现在没有远端可推，直接退。
   *
   * 脏数据（未保存草稿）的保存确认仍然保留：那是纯本地行为，与同步无关。
   */
  async function syncThenExit() {
    setDirtyTabs([]);
    await exit(0);
  }

  async function handleSaveAndExit() {
    setExiting(true);
    const { getDraft, clearDraft } = useTabsStore.getState();
    const failed: string[] = [];
    for (const tab of dirtyTabs) {
      const draft = getDraft(tab.id);
      if (!draft || !draft.title.trim()) {
        failed.push(tab.title || "未命名");
        continue;
      }
      try {
        await noteApi.update(tab.id, { title: draft.title.trim(), content: draft.content });
        clearDraft(tab.id);
      } catch (e) {
        failed.push(`${tab.title || "未命名"}（${e}）`);
      }
    }
    if (failed.length > 0) {
      setExiting(false);
      message.error(`${failed.length} 条笔记保存失败，已取消退出：${failed.join("；")}`);
      // 重新查一次 dirty 列表（有些可能保存成功了）
      setDirtyTabs(useTabsStore.getState().getDirtyTabs());
      return;
    }
    // 保存成功 → 清空 dirty 弹窗 → 走同步阶段
    setDirtyTabs([]);
    await syncThenExit();
    setExiting(false);
  }

  async function handleDiscardAndExit() {
    setExiting(true);
    setDirtyTabs([]);
    // 放弃修改也要 push DB 已有数据（用户可能在别的设备已经改过这些笔记的更老版本）
    await syncThenExit();
    setExiting(false);
  }

  function handleCancel() {
    if (exiting) return;
    setDirtyTabs([]);
  }

  return (
    <>
      {/* 主弹窗：脏数据保存确认（保存/放弃/取消） */}
      <Modal
        open={open}
        title={`有 ${dirtyTabs.length} 条笔记尚未保存`}
        onCancel={handleCancel}
        mask={{ closable: !exiting }}
        closable={!exiting}
        footer={[
          <Button key="discard" danger disabled={exiting} onClick={handleDiscardAndExit}>
            放弃修改并退出
          </Button>,
          <Button key="cancel" disabled={exiting} onClick={handleCancel}>
            取消
          </Button>,
          <Button key="save" type="primary" loading={exiting} onClick={handleSaveAndExit}>
            保存全部并退出
          </Button>,
        ]}
      >
        <p style={{ marginTop: 0 }}>退出后未保存的修改将丢失。请选择操作：</p>
        <List
          size="small"
          bordered
          dataSource={dirtyTabs}
          renderItem={(t) => <List.Item>{t.title || "未命名"}</List.Item>}
          style={{ maxHeight: 200, overflow: "auto" }}
        />
      </Modal>

    </>
  );
}
