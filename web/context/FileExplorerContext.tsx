"use client";

import { Breadcrumb } from "@/lib/types/generated/Breadcrumb";
import { createContext, ReactNode, useContext, useState, useMemo } from "react";

interface FileContextType {
  currentPath: string;
  breadcrumbs: Breadcrumb[];
  setBreadcrumbs: (crumbs: Breadcrumb[]) => void;
  getPathFromUrl: () => string;
}

const FileContext = createContext<FileContextType>({
  currentPath: "",
  breadcrumbs: [],
  setBreadcrumbs: () => {},
  getPathFromUrl: () => "",
});

export function FileProvider({
  children,
  initialPath,
}: {
  children: ReactNode;
  initialPath: string;
}) {
  const [breadcrumbs, setBreadcrumbs] = useState<Breadcrumb[]>([]);

  const value = useMemo(() => ({
    currentPath: initialPath,
    breadcrumbs,
    setBreadcrumbs,
    getPathFromUrl: () => initialPath,
  }), [initialPath, breadcrumbs]); 

  return (
    <FileContext.Provider value={value}>
      {children}
    </FileContext.Provider>
  );
}

export const useFile = () => {
  const context = useContext(FileContext);
  if (!context) {
    console.error("useFile was used outside of FileProvider");
  }
  return context;
};