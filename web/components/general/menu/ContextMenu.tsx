import { createPortal } from "react-dom";
import styles from "./ContextMenu.module.scss";
import { useEffect, useLayoutEffect, useRef } from "react";

interface MenuProps {
  x: number;
  y: number;
  children: React.ReactNode;
}

export const ContextMenu = ({ x, y, children }: MenuProps) => {
  const menuRef = useRef<HTMLDivElement>(null);

  function updateMenuPosition(x: number, y: number) {
    const el = menuRef.current;
    if (!el) return;

    const rect = el.getBoundingClientRect();
    const viewportHeight = window.innerHeight;
    const viewportWidth = window.innerWidth;

    let finalTop = y;
    let finalLeft = x;

    if (y + rect.height + 10 > viewportHeight) finalTop = y - rect.height;
    if (x + rect.width + 10 > viewportWidth) finalLeft = x - rect.width;

    el.style.top = `${Math.max(10, finalTop)}px`;
    el.style.left = `${Math.max(10, finalLeft)}px`;
    el.style.visibility = "visible";
  }
  useLayoutEffect(() => {
    updateMenuPosition(x, y);
  }, [x, y]);

  useEffect(() => {
    window.addEventListener("resize", (event: UIEvent) => {
      updateMenuPosition(x, y);
    });
    return () => {
      window.removeEventListener("resize", () => updateMenuPosition(x, y));
    };
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
    document.body,
  );
};
