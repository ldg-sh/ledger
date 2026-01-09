import { DynamicIcon } from "lucide-react/dynamic";
import styles from "./GlyphButton.module.scss";
import { useEffect, useRef, useState } from "react";
import { ColorInfo } from "@/lib/util/color";
import { cn } from "@/lib/util/class";

interface GlyphButtonProps {
  glyph?: string;
  rotate?: boolean;
  size: number;
  color: ColorInfo;
  fullSize?: number;
}

export default function GlyphButton({
  glyph,
  rotate,
  size,
  color,
  fullSize = 40,
}: GlyphButtonProps) {
  const [rotated, setRotated] = useState(false);
  const buttonRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    buttonRef?.current?.style.setProperty(
      "--color-button-background-active",
      color.backgroundHover
    );
    buttonRef?.current?.style.setProperty(
      "--color-button-background-selected",
      color.backgroundActive
    );
    buttonRef?.current?.style.setProperty(
      "--color-background",
      color.background
    );
    buttonRef?.current?.style.setProperty(
      "--color-foreground",
      color.foreground
    );
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
