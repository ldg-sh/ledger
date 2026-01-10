import { DynamicIcon } from "lucide-react/dynamic";
import styles from "./GlyphButton.module.scss";
import { useRef, useState } from "react";
import { cn } from "@/lib/util/class";

interface GlyphButtonProps {
  glyph?: string;
  rotate?: boolean;
  size: number;
  danger?: boolean;
  fullSize?: number;
}

export default function GlyphButton({
  glyph,
  rotate,
  size,
  danger,
  fullSize = 40,
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
        name={glyph || ("ban" as any)}
        size={size}
        className={cn(styles.glyphIcon, rotated && styles.rotated)}
        style={{
          width: size,
          height: size,
        }}
      />
    </button>
  );
}
