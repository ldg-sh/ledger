import { DynamicIcon } from "lucide-react/dynamic";
import styles from "./ContextMenuItem.module.scss";
import { cn } from "@/lib/util/class";
import { useRef } from "react";

interface ContextMenuItemProps {
  label: string;
  glyph: string;
  onClick: () => void;
  destructive?: boolean;
  disabled?: boolean;
  hotkey?: string;
}

export default function ContextMenuItem({
  label,
  glyph,
  onClick,
  destructive = false,
  disabled = false,
  hotkey,
}: ContextMenuItemProps) {
  let button = useRef<HTMLButtonElement>(null);

  if (hotkey) {
    if (navigator.userAgent.toUpperCase().includes("MAC")) {
      hotkey = hotkey
        .replace("Ctrl", "⌘")
        .replace("Alt", "⌥")
        .replace("Shift", "⇧");
    } else {
      hotkey = hotkey
        .replace("⌘", "Ctrl")
        .replace("⌥", "Alt")
        .replace("⇧", "Shift");
    }
  }

  return (
    <button
      onClick={(e) => {
        e.stopPropagation();
        onClick();
      }}
      ref={button}
      className={cn(
        styles.contextMenuItem,
        destructive && styles.destructive,
        disabled && styles.disabled
      )}
    >
      <div className={styles.left}>
        <DynamicIcon name={glyph as any} className={styles.icon} />
        {label}
      </div>
      {hotkey && <div className={styles.hotkey}>{hotkey}</div>}
    </button>
  );
}
