"use client";

import { Breadcrumb } from "@/lib/types/generated/Breadcrumb";
import { ListFileElement } from "@/lib/types/generated/ListFileElement";
import { usePathname, useSearchParams, useRouter } from "next/navigation";
import { createStore, get, set } from "idb-keyval";
import {
  createContext,
  ReactNode,
  useContext,
  useState,
  useMemo,
  Dispatch,
  SetStateAction,
  useCallback,
  useEffect,
} from "react";
import { listFiles } from "@/lib/api/file";
import { useSort } from "./SortContext";

const ledgerStore = createStore("ledger", "folder-cache");

interface FileListData {
  folders: ListFileElement[];
  files: ListFileElement[];
}

interface FileContextType {
  currentPath: string;
  breadcrumbs: Breadcrumb[];
  fileData: FileListData;
  folderCache: Record<string, FileListData>;
  isHydrated: boolean;
  setBreadcrumbs: Dispatch<SetStateAction<Breadcrumb[]>>;
  setFileData: Dispatch<SetStateAction<FileListData>>;
  setFolderCache: Dispatch<SetStateAction<Record<string, FileListData>>>;
  getPathFromUrl: () => string;
  gotoPath: (id: string) => void;
  prefetchFolder: (id: string) => void;
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
  const [folderCache, setFolderCache] = useState<Record<string, FileListData>>(
    {},
  );
  const [isHydrated, setIsHydrated] = useState(false);

  const searchParams = useSearchParams();
  const router = useRouter();
  const pathname = usePathname();
  const sort = useSort().sort;

  const currentFolderId = searchParams.get("folder") || "";

  useEffect(() => {
    async function initCache() {
      try {
        const saved = await get<Record<string, FileListData>>(
          "fc_cache",
          ledgerStore,
        );
        if (saved) {
          setFolderCache(saved);
        }
      } catch (e) {
        console.error("IndexedDB Load Error:", e);
      } finally {
        setIsHydrated(true);
      }
    }
    initCache();
  }, []);

  useEffect(() => {
    if (!isHydrated) return;

    const timer = setTimeout(async () => {
      try {
        await set("fc_cache", folderCache, ledgerStore);
      } catch (e) {
        console.error("IndexedDB Save Error:", e);
      }
    }, 1000);

    return () => clearTimeout(timer);
  }, [folderCache, isHydrated]);

  const fileData = useMemo(() => {
    if (!isHydrated) return { folders: [], files: [] };
    return folderCache[currentFolderId] || { folders: [], files: [] };
  }, [folderCache, currentFolderId, isHydrated]);

  const setFileData = useCallback(
    (newData: FileListData | ((prev: FileListData) => FileListData)) => {
      setFolderCache((prev) => {
        const data =
          typeof newData === "function"
            ? newData(prev[currentFolderId] || { folders: [], files: [] })
            : newData;
        return {
          ...prev,
          [currentFolderId]: data,
        };
      });
    },
    [currentFolderId],
  );

  const gotoPath = useCallback(
    (id: string) => {
      if (currentFolderId === id) return;

      if (id === "") {
        setBreadcrumbs([]);
        router.push(`${pathname}`);
      } else {
        const params = new URLSearchParams(searchParams.toString());
        params.set("folder", id);
        router.push(`${pathname}?${params.toString()}`);
      }
    },
    [pathname, router, searchParams, currentFolderId],
  );

  const prefetchFolder = useCallback(
    async (id: string) => {
      if (folderCache[id]) return;

      try {
        const cached = await get(id, ledgerStore);
        if (cached) {
          setFolderCache((prev) => ({ ...prev, [id]: cached }));
          return;
        }

        const response = await listFiles(id, sort, 0, 75);

        setFolderCache((prev) => ({ ...prev, [id]: response }));
        await set(id, response, ledgerStore);
      } catch (e) {
        console.warn("Prefetch failed", e);
      }
    },
    [folderCache, sort],
  );

  const value = useMemo(
    () => ({
      currentPath: initialPath,
      breadcrumbs,
      setBreadcrumbs,
      fileData,
      setFileData,
      folderCache,
      setFolderCache,
      isHydrated,
      getPathFromUrl: () => initialPath,
      gotoPath,
      prefetchFolder,
    }),
    [
      initialPath,
      breadcrumbs,
      fileData,
      folderCache,
      setFileData,
      gotoPath,
      isHydrated,
      prefetchFolder
    ],
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
