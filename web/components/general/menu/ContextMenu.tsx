import { createPortal } from "react-dom";
import styles from "./ContextMenu.module.scss";
import { motion } from "motion/react";
import { useLayoutEffect, useRef, useState } from "react";

interface MenuProps {
  x: number;
  y: number;
  children: React.ReactNode;
}

export const ContextMenu = ({ x, y, children }: MenuProps) => {
  const menuRef = useRef<HTMLDivElement>(null);
  const [coords, setCoords] = useState({ top: y, left: x });

  useLayoutEffect(() => {
    if (menuRef.current) {
      const rect = menuRef.current.getBoundingClientRect();
      const viewportHeight = window.innerHeight;
      const viewportWidth = window.innerWidth;

      let finalTop = y;
      let finalLeft = x;

      if (y + rect.height > viewportHeight) {
        finalTop = y - rect.height;
      }

      if (x + rect.width > viewportWidth) {
        finalLeft = x - rect.width;
      }

      finalTop = Math.max(10, finalTop);
      finalLeft = Math.max(10, finalLeft);

      setCoords({ top: finalTop, left: finalLeft });
    }
  }, [x, y]);

  return createPortal(
    <motion.div
      ref={menuRef}
      style={{
        top: coords.top,
        left: coords.left,
        position: "fixed",
        visibility: menuRef.current ? "visible" : "hidden",
        transition: "top 0.1s ease, left 0.1s ease",
      }}
      className={styles.contextMenu}
      initial={{ opacity: 0, transform: "translateY(-10px)" }}
      animate={{ opacity: 1, transform: "translateY(0)" }}
      exit={{ opacity: 0, transform: "translateY(-10px)" }}
      transition={{ duration: 0.15 }}
    >
      {children}
    </motion.div>,
    document.body,
  );
};
