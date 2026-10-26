/**
 * 语音输入按钮 —— **删减占位**。
 *
 * 原 kb 版本接 ASR（录音 + 转写），随删减一并移除（services/asr 已删）。
 * 保留组件与 props 形状、渲染 null：编辑器 / 搜索 / 任务等处有 11 个调用点，
 * 逐一拆 JSX 的风险大于收益；渲染空节点后 UI 自然不再出现麦克风图标。
 *
 * 将来若恢复语音输入，在此重新实现（后端需恢复 services/asr 与 commands/asr）。
 */
interface MicButtonProps {
  size?: "small" | "middle" | "large";
  stripTrailingPunctuation?: boolean;
  onTranscribed?: (text: string) => void;
  className?: string;
  title?: string;
}

export function MicButton(_props: MicButtonProps) {
  return null;
}

export default MicButton;
