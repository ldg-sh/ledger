import { cn } from "@/lib/util/class";
import { DynamicIcon, IconName } from "lucide-react/dynamic";
import { useRef, useState } from "react";
import styles from "./GlyphButton.module.scss";

interface GlyphButtonProps {
  glyph?: string;
  rotate?: boolean;
  size: number;
  danger?: boolean;
  color?: string;
  fullSize?: string;
}

export default function GlyphButton({
  glyph,
  rotate,
  size,
  danger,
  color,
  fullSize,
}: GlyphButtonProps) {
  const [rotated, setRotated] = useState(false);
  const buttonRef = useRef<HTMLButtonElement>(null);

  return (
    <button
      className={styles.glyphButton}
      ref={buttonRef}
      onClick={() => {
        if (rotate) {
          setRotated(!rotated);
        }
      }}
      style={{ width: fullSize || "unset", height: fullSize || "unset" }}
    >
      <DynamicIcon
        name={(glyph as IconName) || "ban"}
        size={size}
        color={color}
        className={cn(
          styles.glyphIcon,
          rotated && styles.rotated,
          danger && styles.danger,
        )}
      />
    </button>
  );
}
