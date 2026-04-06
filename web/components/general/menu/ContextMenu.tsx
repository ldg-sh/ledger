import { createPortal } from "react-dom";
import styles from "./ContextMenu.module.scss";
import { useLayoutEffect, useRef } from "react";

interface MenuProps {
  x: number;
  y: number;
  children: React.ReactNode;
}

export const ContextMenu = ({ x, y, children }: MenuProps) => {
  const menuRef = useRef<HTMLDivElement>(null);

  useLayoutEffect(() => {
    const el = menuRef.current;
    if (!el) return;

    const rect = el.getBoundingClientRect();
    const viewportHeight = window.innerHeight;
    const viewportWidth = window.innerWidth;

    let finalTop = y;
    let finalLeft = x;

    if (y + rect.height > viewportHeight) finalTop = y - rect.height;
    if (x + rect.width > viewportWidth) finalLeft = x - rect.width;

    el.style.top = `${Math.max(10, finalTop)}px`;
    el.style.left = `${Math.max(10, finalLeft)}px`;
    el.style.visibility = "visible";
  }, [x, y]);

  return createPortal(
    <div
      ref={menuRef}
      style={{
        top: y, 
        left: x,
        position: "fixed",
        visibility: "hidden",
      }}
      className={styles.contextMenu}
    >
      {children}
    </div>,
    document.body
  );
};