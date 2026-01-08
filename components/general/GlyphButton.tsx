import { DynamicIcon } from "lucide-react/dynamic";
import styles from "./GlyphButton.module.scss";
import { useState } from "react";

interface GlyphButtonProps {
  glyph?: string;
  rotate?: boolean;
}

export default function GlyphButton({ glyph, rotate }: GlyphButtonProps) {
  const [rotated, setRotated] = useState(false);

  return (
    <button
      className={styles.glyphButton}
      onClick={() => {
        if (rotate) {
          setRotated(!rotated);
        }
      }}
    >
      <DynamicIcon
        name={glyph || ("ban" as any)}
        size={20}
        className={styles.glyphIcon + (rotated ? " " + styles.rotated : "")}
      />
    </button>
  );
}
