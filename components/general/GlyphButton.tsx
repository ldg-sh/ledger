import { DynamicIcon } from "lucide-react/dynamic";
import styles from "./GlyphButton.module.scss";
import { useEffect, useRef, useState } from "react";
import { ColorInfo } from "@/lib/util/color";

interface GlyphButtonProps {
  glyph?: string;
  rotate?: boolean;
  size: number;
  color: ColorInfo;
}

export default function GlyphButton({ glyph, rotate, size, color }: GlyphButtonProps) {
  const [rotated, setRotated] = useState(false);
  const buttonRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    buttonRef?.current?.style.setProperty("--color-button-background-active", color.backgroundHover);
    buttonRef?.current?.style.setProperty("--color-button-background-selected", color.backgroundActive);
    buttonRef?.current?.style.setProperty("--color-background", color.background);
    buttonRef?.current?.style.setProperty("--color-foreground", color.foreground);
  }, [color]);

  return (
    <button
      className={styles.glyphButton}
      ref={buttonRef}
      onClick={() => {
        if (rotate) {
          setRotated(!rotated);
        }
      }}
    >
      <DynamicIcon
        name={glyph || ("ban" as any)}
        size={size}
        className={styles.glyphIcon + (rotated ? " " + styles.rotated : "")}
        style={{
          width: size,
          height: size
        }}
      />
    </button>
  );
}
