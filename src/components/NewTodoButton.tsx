import { useState } from "react";
import { Button } from "antd";
import type { SizeType } from "antd/es/config-provider/SizeContext";
import { CheckSquare } from "lucide-react";

import { CreateTaskModal } from "@/components/tasks/CreateTaskModal";

import { useAppStore } from "@/store";

interface Props {
  /** 块级占满父容器宽度（首页快捷操作大按钮用） */
  block?: boolean;
  /** 主按钮文字，默认"添加待办" */
  label?: string;
  /** 外层样式扩展 */
  style?: React.CSSProperties;
  /** 按钮尺寸（透传给内部 Button），默认 middle；首页大按钮用 large */
  size?: SizeType;
  /** 保存成功后回调（落库后宿主页可刷新列表/统计） */
  onSaved?: () => void;
}

/**
 * "+ 添加待办"按钮：点击直接弹 CreateTaskModal 加单条。
 *
 * 与 NewNoteButton 同款交互，Modal 内置在组件里，宿主只需挂一次。
 */
export function NewTodoButton({
  block = false,
  label = "添加待办",
  style,
  size,
  onSaved,
}: Props) {
  const [createOpen, setCreateOpen] = useState(false);

  const refreshTaskStats = useAppStore((s) => s.refreshTaskStats);

  function handleSaved() {
    refreshTaskStats();
    onSaved?.();
  }

  return (
    <>
      <Button
        type="primary"
        size={size}
        icon={<CheckSquare size={14} />}
        onClick={() => setCreateOpen(true)}
        title="新建一条待办"
        block={block}
        style={style}
      >
        {label}
      </Button>

      <CreateTaskModal
        open={createOpen}
        onClose={() => setCreateOpen(false)}
        onSaved={() => {
          setCreateOpen(false);
          handleSaved();
        }}
      />
    </>
  );
}
