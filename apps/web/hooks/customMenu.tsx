import { useState, useEffect, useCallback, useContext } from "react";
import { MenuContext } from "../context/MenuContext";

const MENU_WIDTH = 200;

export const useCustomMenu = (menuId: string) => {
  const context = useContext(MenuContext);

  if (!context) {
    throw new Error("useCustomMenu must be used within a MenuProvider");
  }

  const { activeMenuId, openMenu, closeMenu } = context;
  const [position, setPosition] = useState({ x: 0, y: 0 });

  const isVisible = activeMenuId === menuId;

  const showMenu = useCallback((event: React.MouseEvent) => {
    event.preventDefault();

    if (event.pageX + MENU_WIDTH + 100 > window.innerWidth) {
      setPosition({ x: event.pageX - MENU_WIDTH, y: event.pageY });
    } else {
      setPosition({ x: event.pageX, y: event.pageY });
    }
    
    openMenu(menuId);
  }, [menuId, openMenu]);

  useEffect(() => {
    if (!isVisible) return;

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        closeMenu();
      }
    };

    const handleClickOutside = () => {
      closeMenu();
    };

    document.addEventListener("click", handleClickOutside);
    document.addEventListener("keydown", handleKeyDown);

    return () => {
      document.removeEventListener("click", handleClickOutside);
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [isVisible, closeMenu]);

  return { 
    visible: isVisible, 
    position, 
    showMenu, 
    hideMenu: closeMenu 
  };
};