"use client";

import { copyFile, copyMultipleFiles, listFiles } from "@/lib/api/file";
import Row from "./Row";
import { usePathname } from "next/navigation";
import { useEffect, useState, useCallback } from "react";
import { extractPathFromUrl } from "@/lib/util/url";

interface File {
  fileId: string;
  fileName: string;
  fileSize: number;
  fileType: string;
  createdAt?: string;
  path: string;
}

interface FileListData {
  folders: File[];
  files: File[];
}

export default function FileList() {
  const pathname = usePathname();
  const [data, setData] = useState<FileListData | null>(null);
  const [selectedFiles, setSelectedFiles] = useState<Set<string>>(new Set());
  const [lastDeliberateClick, setLastDeliberateClick] = useState<string | null>(
    null
  );

  const getAllFileIds = useCallback((): string[] => {
    if (!data) return [];
    return [
      ...data.folders.map((f) => f.fileId),
      ...data.files.map((f) => f.fileId),
    ];
  }, [data]);

  const getClipboardData = async (): Promise<string[]> => {
    try {
      const text = await navigator.clipboard.readText();
      return text
        .split("\n")
        .map((id) => id.trim())
        .filter((id) => id.length > 0);
    } catch (err) {
      console.error("Failed to read clipboard contents:", err);
      return [];
    }
  };

  const writeToClipboard = async (text: string): Promise<void> => {
    try {
      await navigator.clipboard.writeText(text);
    } catch (err) {
      console.error("Could not copy text:", err);
    }
  };

  const copyFileIdsToClipboard = useCallback(async () => {
    if (selectedFiles.size > 0) {
      const fileIdsString = Array.from(selectedFiles).join("\n");
      await writeToClipboard(fileIdsString);
    } else if (lastDeliberateClick) {
      await writeToClipboard(lastDeliberateClick);
    }
  }, [selectedFiles, lastDeliberateClick]);

  const copyFileIdToClipboard = useCallback(
    async (fileId: string) => {
      if (selectedFiles.size > 1) {
        const fileIdsString = Array.from(selectedFiles).join("\n");
        await writeToClipboard(fileIdsString);
        return;
      }
      await writeToClipboard(fileId);
    },
    [selectedFiles]
  );

  const pasteFileIdsFromClipboard = useCallback(
    async (ids: string[]) => {
      let fileIdsToCopy: string[] = [];
      console.log("PASTE IDS", ids);

      ids.forEach((id) => {
        let file = data?.files.find((file) => file.fileId === id);
        if (file) {
          fileIdsToCopy.push(id);
        }
      });

      console.log("FILE IDS TO COPY", fileIdsToCopy);

      const fileIds = await copyMultipleFiles(
        fileIdsToCopy,
        extractPathFromUrl(pathname)
      );

      const event = new CustomEvent("refresh-file-list", {
        detail: () => {
          setSelectedFiles(new Set(fileIds.filter(Boolean)));
        },
      });
      window.dispatchEvent(event);
    },
    [pathname, data]
  );

  const pasteFileIdFromClipboard = useCallback(
    async (id: string) => {
      let file = data?.files.find((file) => file.fileId === id);

      let newId = "";

      if (file) {
        newId = await copyFile(id.trim(), extractPathFromUrl(pathname));
      }

      const event = new CustomEvent("refresh-file-list", {
        detail: () => {
          setSelectedFiles(newId ? new Set([newId]) : new Set());
        },
      });
      window.dispatchEvent(event);
    },
    [data, pathname]
  );

  const handleSelectAll = useCallback(() => {
    setSelectedFiles(new Set(getAllFileIds()));
  }, [getAllFileIds]);

  const handleSelectUpper = useCallback(() => {
    if (!lastDeliberateClick) return;

    const allFileIds = getAllFileIds();
    const lastIndex = allFileIds.indexOf(lastDeliberateClick);

    if (lastIndex > 0) {
      const newSelected = new Set(selectedFiles);
      newSelected.add(allFileIds[lastIndex - 1]);
      setSelectedFiles(newSelected);
      setLastDeliberateClick(allFileIds[lastIndex - 1]);
    }
  }, [lastDeliberateClick, getAllFileIds, selectedFiles]);

  const handleSelectLower = useCallback(() => {
    if (!lastDeliberateClick) return;

    const allFileIds = getAllFileIds();
    const lastIndex = allFileIds.indexOf(lastDeliberateClick);

    if (lastIndex < allFileIds.length - 1) {
      const newSelected = new Set(selectedFiles);
      newSelected.add(allFileIds[lastIndex + 1]);
      setSelectedFiles(newSelected);
      setLastDeliberateClick(allFileIds[lastIndex + 1]);
    }
  }, [lastDeliberateClick, getAllFileIds, selectedFiles]);

  const clearSelection = useCallback(() => {
    setSelectedFiles(new Set());
    setLastDeliberateClick(null);
  }, []);

  const handleRowClick = useCallback(
    (
      fileId: string,
      selected: boolean,
      isShiftKey: boolean,
      isCommandKey: boolean
    ) => {
      setSelectedFiles((prevSelected) => {
        const newSelected = new Set(prevSelected);

        if (isCommandKey) {
          selected ? newSelected.delete(fileId) : newSelected.add(fileId);
        } else if (isShiftKey && lastDeliberateClick) {
          newSelected.clear();
          const allFileIds = getAllFileIds();
          const startIndex = allFileIds.indexOf(lastDeliberateClick);
          const endIndex = allFileIds.indexOf(fileId);

          if (startIndex > -1 && endIndex > -1) {
            const [from, to] =
              startIndex < endIndex
                ? [startIndex, endIndex]
                : [endIndex, startIndex];

            for (let i = from; i <= to; i++) {
              newSelected.add(allFileIds[i]);
            }
          }
        } else {
          newSelected.clear();
          newSelected.add(fileId);
          setLastDeliberateClick(fileId);
        }

        return newSelected;
      });
    },
    [lastDeliberateClick, getAllFileIds]
  );

  const loadData = useCallback(async () => {
    const res = await listFiles(extractPathFromUrl(pathname));
    setData(res);
  }, [pathname]);

  const refreshFileList = useCallback(
    async (event: Event) => {
      await loadData();

      if (event instanceof CustomEvent && typeof event.detail === "function") {
        event.detail();
      }
    },
    [loadData]
  );

  useEffect(() => {
    const handleKeyDown = async (event: KeyboardEvent) => {
      const isMod = event.metaKey || event.ctrlKey;

      if (isMod && event.key === "a") {
        event.preventDefault();
        handleSelectAll();
      } else if (event.key === "Escape") {
        event.preventDefault();
        clearSelection();
      } else if (event.key === "ArrowUp" && event.shiftKey) {
        event.preventDefault();
        handleSelectUpper();
      } else if (event.key === "ArrowDown" && event.shiftKey) {
        event.preventDefault();
        handleSelectLower();
      } else if (isMod && event.key === "c") {
        event.preventDefault();
        await copyFileIdsToClipboard();
      } else if (isMod && event.key === "v") {
        event.preventDefault();
        const clipboard = await getClipboardData();

        if (clipboard.length === 1) {
          await pasteFileIdFromClipboard(clipboard[0]);
        } else if (clipboard.length > 1) {
          await pasteFileIdsFromClipboard(clipboard);
        }
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [
    handleSelectAll,
    clearSelection,
    handleSelectUpper,
    handleSelectLower,
    copyFileIdsToClipboard,
    pasteFileIdFromClipboard,
    pasteFileIdsFromClipboard,
  ]);

  useEffect(() => {
    const handleCopyEvent = async (event: Event) => {
      if (event instanceof CustomEvent && event.detail?.fileId) {
        await copyFileIdToClipboard(event.detail.fileId);
      } else {
        await copyFileIdsToClipboard();
      }
      await refreshFileList(event);
    };

    document.addEventListener("copy-file-ids", handleCopyEvent);
    return () => document.removeEventListener("copy-file-ids", handleCopyEvent);
  }, [copyFileIdToClipboard, copyFileIdsToClipboard, refreshFileList]);

  useEffect(() => {
    const handlePasteEvent = async (event: Event) => {
      const clipboard = await getClipboardData();

      if (clipboard.length === 1) {
        await pasteFileIdFromClipboard(clipboard[0]);
      } else if (clipboard.length > 1) {
        await pasteFileIdsFromClipboard(clipboard);
      }

      await refreshFileList(event);
    };

    document.addEventListener("paste-file-ids", handlePasteEvent);
    return () =>
      document.removeEventListener("paste-file-ids", handlePasteEvent);
  }, [pasteFileIdFromClipboard, pasteFileIdsFromClipboard, refreshFileList]);

  useEffect(() => {
    loadData();
    window.addEventListener("refresh-file-list", refreshFileList);
    return () =>
      window.removeEventListener("refresh-file-list", refreshFileList);
  }, [loadData, refreshFileList]);

  return (
    <div>
      {data?.folders.map((folder) => (
        <Row
          key={folder.fileId}
          fileId={folder.fileId}
          fileName={folder.fileName}
          fileSize={folder.fileSize}
          fileType="folder"
          folder={true}
          clickCallback={handleRowClick}
          selected={selectedFiles.has(folder.fileId)}
        />
      ))}
      {data?.files.map((file) => (
        <Row
          key={file.fileId}
          fileName={file.fileName}
          fileSize={file.fileSize}
          fileType={file.fileType}
          fileId={file.fileId}
          createdAt={file.createdAt}
          clickCallback={handleRowClick}
          selected={selectedFiles.has(file.fileId)}
        />
      ))}
      <div style={{ height: "100px", width: "100%" }} />
    </div>
  );
}
