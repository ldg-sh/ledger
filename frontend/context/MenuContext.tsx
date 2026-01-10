"use client";

import React, { createContext, useState, useCallback } from "react";

type MenuContextType = {
  activeMenuId: string | null;
  openMenu: (id: string) => void;
  closeMenu: () => void;
};

export const MenuContext = createContext<MenuContextType | undefined>(undefined);

export const MenuProvider = ({ children }: { children: React.ReactNode }) => {
  const [activeMenuId, setActiveMenuId] = useState<string | null>(null);

  const openMenu = useCallback((id: string) => {
    setActiveMenuId(id);
  }, []);

  const closeMenu = useCallback(() => {
    setActiveMenuId(null);
  }, []);

  return (
    <MenuContext.Provider value={{ activeMenuId, openMenu, closeMenu }}>
      {children}
    </MenuContext.Provider>
  );
};