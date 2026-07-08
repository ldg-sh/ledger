import styles from "./ContextMenuItem.module.scss";
import { cn } from "@/lib/util/class";
import { useRef } from "react";
import Spinner from "@/components/svg/Spinner";
import { ICON_REGISTRY, IconName } from "@/lib/util/icon";
import { Check } from "lucide-react";
import { useEffect, useState } from "react";


interface ContextMenuItemProps {
  label: string;
  glyph: IconName;
  onClick: () => void;
  destructive?: boolean;
  disabled?: boolean;
  hotkey?: string;
  isLoading?: boolean;
  isSuccess?: boolean;
}

function useWindowSize() {
  const [size, setSize] = useState<{ width: number; height: number } | null>(null);

  useEffect(() => {
    const update = () =>
      setSize({ width: window.innerWidth, height: window.innerHeight });

    update();
    window.addEventListener("resize", update);
    return () => window.removeEventListener("resize", update);
  }, []);

  return size;
}

export default function ContextMenuItem({
  label,
  glyph,
  onClick,
  destructive = false,
  disabled = false,
  hotkey,
  isLoading = false,
  isSuccess = false,
}: ContextMenuItemProps) {
  const button = useRef<HTMLButtonElement>(null);
  const IconComponent = ICON_REGISTRY[glyph];
  const windowSize = useWindowSize();

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
        disabled && styles.disabled,
      )}
    >
      <div className={styles.left}>
        {isLoading ? (
          <Spinner height={(windowSize?.width ?? 1000) > 720 ? 16 : 12} destructive={destructive} />
        ) : (
          isSuccess ? <Check size={16} className={styles.icon} /> : IconComponent && <IconComponent className={styles.icon} size={16} />
        )}
        {label}
      </div>
      {hotkey && <div className={styles.hotkey}>{hotkey}</div>}
    </button>
  );
}
