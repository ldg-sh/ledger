"use client";

import { Breadcrumb } from "@/lib/types/generated/Breadcrumb";
import { useSearchParams } from "next/navigation";
import { createContext, ReactNode, useContext, useState, useMemo } from "react";

interface FileContextType {
  currentPath: string;
  breadcrumbs: Breadcrumb[];
  setBreadcrumbs: (crumbs: Breadcrumb[]) => void;
  getPathFromUrl: () => string;
}

const FileContext = createContext<FileContextType | undefined>(undefined);

export function FileProvider({ children }: { children: ReactNode }) {
  const [breadcrumbs, setBreadcrumbs] = useState<Breadcrumb[]>([]);
  const searchParams = useSearchParams();

  const currentPath = useMemo(() => {
    return searchParams.get("folder") || "";
  }, [searchParams]);

  const getPathFromUrl = () => currentPath;

  return (
    <FileContext.Provider
      value={{
        currentPath,
        breadcrumbs,
        setBreadcrumbs,
        getPathFromUrl,
      }}
    >
      {children}
    </FileContext.Provider>
  );
}

export const useFile = () => {
  const context = useContext(FileContext);
  if (!context) {
    throw new Error("useFile must be used within a FileProvider");
  }
  return context;
};