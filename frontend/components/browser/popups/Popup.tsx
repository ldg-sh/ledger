"use client";

import { defaultColor } from "@/lib/util/color";
import styles from "./Popup.module.scss";
import GlyphButton from "@/components/general/GlyphButton";
import { easeOut, motion } from "framer-motion";
import { useEffect } from "react";

interface PopupProps {
  children: React.ReactNode;
  onClosePopup?: () => void;
}

export default function Popup({ children, onClosePopup }: PopupProps) {
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        onClosePopup && onClosePopup();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [onClosePopup]);

  return (
    <div>
      <motion.div
        className={styles.popupBackdrop}
        onClick={() => {
          onClosePopup && onClosePopup();
        }}
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        transition={{ ease: easeOut, duration: 0.2 }}
      />
      <motion.div
        className={styles.popupContainer}
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        transition={{ ease: easeOut, duration: 0.2 }}
      >
        <motion.div
          className={styles.popupChildren}
          initial={{ translateY: 10 }}
          animate={{ translateY: 0 }}
          exit={{ translateY: -10 }}
          transition={{ ease: easeOut, duration: 0.2 }}
        >
          <div
            className={styles.close}
            onClick={() => {
              onClosePopup && onClosePopup();
            }}
          >
            <GlyphButton
              glyph={"x"}
              size={16}
              fullSize={30}
              color={defaultColor}
            />
          </div>
          {children}
        </motion.div>
      </motion.div>
    </div>
  );
}
