import { useState, useEffect, useCallback } from "react";

export const useCustomMenu = () => {
  const [visible, setVisible] = useState(false);
  const [position, setPosition] = useState({ x: 0, y: 0 });

  const showMenu = useCallback((event: React.MouseEvent) => {
    event.preventDefault();

    setPosition({ x: event.pageX, y: event.pageY });
    setVisible(true);
  }, []);

  const hideMenu = useCallback(() => {
    setVisible(false);
  }, []);

  useEffect(() => {
    document.addEventListener("click", hideMenu);
    document.addEventListener("keydown", (event) => {
      if (event.key === "Escape") {
        hideMenu();
      }
    });
    return () => {
      () => {
        document.removeEventListener("click", hideMenu);
        document.removeEventListener("keydown", hideMenu);
      };
    };
  }, [hideMenu]);

  return { visible, position, showMenu, hideMenu };
};
