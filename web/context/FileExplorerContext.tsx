"use client";

import { Breadcrumb } from "@/lib/types/generated/Breadcrumb";
import { ListFileElement } from "@/lib/types/generated/ListFileElement";
import { usePathname, useSearchParams, useRouter } from "next/navigation"; // Note: useRouter from navigation for App Router
import {
  createContext,
  ReactNode,
  useContext,
  useState,
  useMemo,
  Dispatch,
  SetStateAction,
  useCallback,
} from "react";

interface FileListData {
  folders: ListFileElement[];
  files: ListFileElement[];
}

interface FileContextType {
  currentPath: string;
  breadcrumbs: Breadcrumb[];
  fileData: FileListData;
  setBreadcrumbs: Dispatch<SetStateAction<Breadcrumb[]>>;
  setFileData: Dispatch<SetStateAction<FileListData>>;
  getPathFromUrl: () => string;
  gotoPath: (id: string) => void;
}

const FileContext = createContext<FileContextType | undefined>(undefined);

export function FileProvider({
  children,
  initialPath,
}: {
  children: ReactNode;
  initialPath: string;
}) {
  const [breadcrumbs, setBreadcrumbs] = useState<Breadcrumb[]>([]);
  const [fileData, setFileData] = useState<FileListData>({
    folders: [],
    files: [],
  });

  const searchParams = useSearchParams();
  const router = useRouter();
  const pathname = usePathname();

  const gotoPath = useCallback(
    (id: string) => {
      const currentFolderId = searchParams.get("folder") || "";
      if (currentFolderId === id) {
        return;
      }

      setFileData({ folders: [], files: [] });

      if (id === "") {
        setBreadcrumbs([]);

        router.push(`${pathname}`);
      } else {
        const params = new URLSearchParams(searchParams.toString());
        params.set("folder", id);

        router.push(`${pathname}?${params.toString()}`);
      }
    },
    [pathname, router, searchParams],
  );

  const value = useMemo(
    () => ({
      currentPath: initialPath,
      breadcrumbs,
      setBreadcrumbs,
      fileData,
      setFileData,
      getPathFromUrl: () => initialPath,
      gotoPath,
    }),
    [initialPath, breadcrumbs, fileData, gotoPath],
  );
  return <FileContext.Provider value={value}>{children}</FileContext.Provider>;
}

export const useFile = () => {
  const context = useContext(FileContext);
  if (!context) {
    throw new Error("useFile must be used within a FileProvider");
  }
  return context;
};
