"use client";

import { copyFiles, listFiles } from "@/lib/api/file";
import Row from "./Row";
import { usePathname } from "next/navigation";
import { useEffect, useState, useCallback, useRef } from "react";
import { extractPathFromUrl } from "@/lib/util/url";
import { useUser } from "@/context/UserContext";
import { useCustomMenu } from "@/hooks/customMenu";
import { AnimatePresence } from "motion/react";
import { ContextMenu } from "../general/menu/ContextMenu";
import ContextMenuItem from "../general/menu/ContextMenuItem";
import RenameFile from "./popups/RenameFile";
import DeleteFile from "./popups/DeleteFile";
import { ListFileElement } from "@/lib/types/generated/ListFileElement";
import { useSort } from "@/context/SortContext";
import { setGlobalLoading, useLoading } from "@/context/LoadingContext";

interface FileListData {
  folders: ListFileElement[];
  files: ListFileElement[];
}

interface FileListProps {
  parentContainerRef?: React.RefObject<HTMLDivElement>;
}

const CHUNK_SIZE = 75;

export default function FileList({ parentContainerRef }: FileListProps) {
  const { sort } = useSort();
  const pathname = usePathname();

  const lastRefreshTime = useRef<number>(0);
  const THROTTLE_MS = 500;

  const [data, setData] = useState<FileListData | null>(null);
  const [selectedFiles, setSelectedFiles] = useState<Set<ListFileElement>>(
    new Set(),
  );
  const [lastDeliberateClick, setLastDeliberateClick] =
    useState<ListFileElement | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [currentOffset, setCurrentOffset] = useState(0);
  const [hasMore, setHasMore] = useState(true);
  const [isRefreshing, setIsRefreshing] = useState(false);

  const [rightClickedFile, setRightClickedFile] =
    useState<ListFileElement | null>(null);

  const [isRenamePopupOpen, setIsRenamePopupOpen] = useState(false);
  const [isDeletePopupOpen, setIsDeletePopupOpen] = useState(false);

  const { visible, position, showMenu, hideMenu } =
    useCustomMenu("file-list-menu");

  const { user, loading: authLoading } = useUser();

  const getAllFiles = useCallback((): ListFileElement[] => {
    if (!data) return [];
    return [...data.folders, ...data.files];
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
      await writeToClipboard(lastDeliberateClick.id);
    }
  }, [selectedFiles, lastDeliberateClick]);

  const copyFileIdToClipboard = useCallback(
    async (fileId: string) => {
      await writeToClipboard(fileId);
    },
    [selectedFiles],
  );

  const pasteFileIdsFromClipboard = useCallback(
    async (ids: string[]) => {
      let fileIdsToCopy: string[] = [];

      ids.forEach((id) => {
        let file = data?.files.find((file) => file.id === id);
        if (file) {
          fileIdsToCopy.push(id);
        }
      });

      const fileIds = await copyFiles(
        fileIdsToCopy,
        extractPathFromUrl(pathname),
      );

      const event = new CustomEvent("refresh-file-list", {
        detail: () => {
          const newlyCopiedFiles = fileIds
            .map((id) => data?.files.find((file) => file.id === id))
            .filter((f): f is ListFileElement => !!f);

          setSelectedFiles(new Set(newlyCopiedFiles));
        },
      });
      window.dispatchEvent(event);
    },
    [pathname, data],
  );

  const pasteFileIdFromClipboard = useCallback(
    async (id: string) => {
      let file = data?.files.find((file) => file.id === id);

      let newId = "";

      if (file) {
        newId = (await copyFiles([id.trim()], extractPathFromUrl(pathname)))[0];
      }

      const event = new CustomEvent("refresh-file-list", {
        detail: () => {
          const newFile = data?.files.find((f) => f.id === newId);

          if (newFile) {
            setSelectedFiles(new Set([newFile]));
          } else {
            setSelectedFiles(new Set());
          }
        },
      });
      window.dispatchEvent(event);
    },
    [data, pathname],
  );

  const handleSelectAll = useCallback(() => {
    setSelectedFiles(new Set(getAllFiles()));
  }, [getAllFiles]);

  const handleSelectUpper = useCallback(() => {
    if (!lastDeliberateClick) return;

    const allFiles = getAllFiles();
    const lastIndex = allFiles.indexOf(lastDeliberateClick);

    if (lastIndex > 0) {
      const newSelected = new Set(selectedFiles);
      newSelected.add(allFiles[lastIndex - 1]);
      setSelectedFiles(newSelected);
      setLastDeliberateClick(allFiles[lastIndex - 1]);
    }
  }, [lastDeliberateClick, getAllFiles, selectedFiles]);

  const handleSelectLower = useCallback(() => {
    if (!lastDeliberateClick) return;

    const allFiles = getAllFiles();
    const lastIndex = allFiles.indexOf(lastDeliberateClick);

    if (lastIndex < allFiles.length - 1) {
      const newSelected = new Set(selectedFiles);
      newSelected.add(allFiles[lastIndex + 1]);
      setSelectedFiles(newSelected);
      setLastDeliberateClick(allFiles[lastIndex + 1]);
    }
  }, [lastDeliberateClick, getAllFiles, selectedFiles]);

  const clearSelection = useCallback(() => {
    setSelectedFiles(new Set());
    setLastDeliberateClick(null);
  }, []);

  const handleRowClick = useCallback(
    (
      file: ListFileElement,
      selected: boolean,
      isShiftKey: boolean,
      isCommandKey: boolean,
    ) => {
      setSelectedFiles((prevSelected) => {
        const newSelected = new Set(prevSelected);

        if (isCommandKey) {
          selected ? newSelected.delete(file) : newSelected.add(file);
        } else if (isShiftKey && lastDeliberateClick) {
          newSelected.clear();
          const allFiles = getAllFiles();
          const startIndex = allFiles.indexOf(lastDeliberateClick);
          const endIndex = allFiles.indexOf(file);

          if (startIndex > -1 && endIndex > -1) {
            const [from, to] =
              startIndex < endIndex
                ? [startIndex, endIndex]
                : [endIndex, startIndex];

            for (let i = from; i <= to; i++) {
              newSelected.add(allFiles[i]);
            }
          }
        } else {
          newSelected.clear();
          newSelected.add(file);
          setLastDeliberateClick(file);
        }

        return newSelected;
      });
    },
    [lastDeliberateClick, getAllFiles],
  );

  const loadData = useCallback(async () => {
    if (authLoading) return;

    setIsLoading(true);
    setGlobalLoading(true);
    try {
      const res = await listFiles(
        extractPathFromUrl(pathname),
        sort,
        0,
        CHUNK_SIZE,
      );

      if (!res.hasMore) {
        setHasMore(false);
      } else {
        setHasMore(true);
      }

      setData(res);
    } finally {
      setIsLoading(false);
      setGlobalLoading(false);
    }
  }, [pathname, authLoading, sort]);

  const loadMoreData = useCallback(async () => {
    if (authLoading || !hasMore || isLoading) return;

    setIsLoading(true);
    setGlobalLoading(true);

    const res = await listFiles(
      extractPathFromUrl(pathname),
      sort,
      currentOffset,
      CHUNK_SIZE,
    );

    if (!res.hasMore) {
      setHasMore(false);
    } else {
      setHasMore(true);
    }

    setData((prevData) => {
      if (!prevData) return res;

      res.files = res.files.filter(
        (newFile) =>
          !prevData.files.some(
            (existingFile) => existingFile.id === newFile.id,
          ),
      );

      res.folders = res.folders.filter(
        (newFolder) =>
          !prevData.folders.some(
            (existingFolder) => existingFolder.id === newFolder.id,
          ),
      );

      return {
        folders: [...prevData.folders, ...res.folders],
        files: [...prevData.files, ...res.files],
        hasMore: res.hasMore,
      };
    });
    setIsLoading(false);
    setGlobalLoading(false);
  }, [pathname, authLoading, sort, currentOffset]);

  useEffect(() => {
    setCurrentOffset(0);

    if (!authLoading && user) {
      loadData();
    }
  }, [loadData, authLoading, user]);

  const refreshFileList = useCallback(
    async (event: Event) => {
      const now = Date.now();

      if (isRefreshing || now - lastRefreshTime.current < THROTTLE_MS) {
        console.log("Refresh throttled");
        return;
      }

      setIsRefreshing(true);
      lastRefreshTime.current = now;

      try {
        await loadData();

        if (
          event instanceof CustomEvent &&
          typeof event.detail === "function"
        ) {
          event.detail();
        } else if (
          event instanceof CustomEvent &&
          typeof event.detail === "string"
        ) {
          const file =
            data?.files.find((f) => f.id === event.detail) ||
            data?.folders.find((f) => f.id === event.detail);

          if (selectedFiles.size === 0 && file) {
            setSelectedFiles(new Set([file]));
          }
        }
      } finally {
        setIsRefreshing(false);
      }
    },
    [isRefreshing, loadData, data, selectedFiles.size],
  );

  useEffect(() => {
    const handleKeyDown = async (event: KeyboardEvent) => {
      const isMod = event.metaKey || event.ctrlKey;

      if (
        event.target instanceof HTMLInputElement ||
        event.target instanceof HTMLTextAreaElement
      ) {
        return;
      }

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
    window.addEventListener("refresh-file-list", refreshFileList);

    return () =>
      window.removeEventListener("refresh-file-list", refreshFileList);
  }, [loadData, refreshFileList, parentContainerRef]);

  useEffect(() => {
    const container = parentContainerRef?.current;
    if (!container) return;

    const handleScroll = () => {
      const { scrollHeight, scrollTop, clientHeight } = container;
      if (scrollHeight - scrollTop - clientHeight < 100 && !isLoading) {
        setCurrentOffset((prev) => prev + CHUNK_SIZE);
      }
    };

    container.addEventListener("scroll", handleScroll);
    return () => container.removeEventListener("scroll", handleScroll);
  }, [parentContainerRef, isLoading]);

  useEffect(() => {
    if (currentOffset > 0) {
      loadMoreData();
    }
  }, [currentOffset, loadMoreData]);

  return (
    <>
      <div
        style={{
          opacity: isLoading ? ".5" : "1",
          transition: "opacity 0.2s",
        }}
        onContextMenu={(event) => {
          const target = event.target as HTMLElement;

          event.preventDefault();

          const rowElement = target.closest("[data-context-boundary]");

          const fileId = rowElement?.getAttribute("data-file-id");
          const fileName = rowElement?.getAttribute("data-file-name");

          let file =
            data?.files.find((f) => f.id === fileId) ||
            data?.folders.find((f) => f.id === fileId);

          if (fileId && fileName) {
            setRightClickedFile(file || null);
            setLastDeliberateClick(file || null);
          } else {
            setLastDeliberateClick(null);
          }

          showMenu(event);
        }}
      >
        {data?.folders.map((folder) => (
          <Row
            key={folder.id}
            fileId={folder.id}
            fileName={folder.file_name}
            fileSize={folder.file_size as unknown as number}
            fileType="folder"
            folder={true}
            clickCallback={handleRowClick}
            selected={selectedFiles.has(folder)}
            file={folder}
          />
        ))}
        {data?.files.map((file) => (
          <Row
            key={file.id}
            fileName={file.file_name}
            fileSize={file.file_size as unknown as number}
            fileType={file.file_type}
            fileId={file.id}
            createdAt={file.created_at}
            clickCallback={handleRowClick}
            selected={selectedFiles.has(file)}
            file={file}
          />
        ))}
        <div style={{ height: "100px", width: "100%" }} />
      </div>
      <AnimatePresence>
        {visible && (
          <div>
            <ContextMenu x={position.x} y={position.y}>
              <ContextMenuItem
                label="Select All"
                glyph="check-check"
                hotkey="CtrlA"
                onClick={() => {
                  handleSelectAll();
                  hideMenu();
                }}
              />
              <ContextMenuItem
                label="Copy"
                glyph="copy"
                hotkey="CtrlC"
                onClick={() => {
                  if (
                    !Array.from(selectedFiles).includes(
                      rightClickedFile as ListFileElement,
                    )
                  ) {
                    copyFileIdToClipboard(rightClickedFile?.id || "");
                  } else {
                    copyFileIdsToClipboard();
                  }

                  hideMenu();
                }}
              />
              <ContextMenuItem
                label="Paste"
                glyph="clipboard-paste"
                hotkey="CtrlV"
                onClick={() => {
                  getClipboardData().then((clipboard) => {
                    if (clipboard.length === 1) {
                      pasteFileIdFromClipboard(clipboard[0]);
                    } else if (clipboard.length > 1) {
                      pasteFileIdsFromClipboard(clipboard);
                    }
                  });
                  hideMenu();
                }}
              />
              {selectedFiles.size === 1 && (
                <ContextMenuItem
                  label="Copy Link"
                  glyph="link"
                  onClick={() => {
                    const fileId = Array.from(selectedFiles)[0].id;
                    copyFileIdToClipboard(fileId);
                    hideMenu();
                  }}
                />
              )}
              <ContextMenuItem
                label="Download"
                glyph="download"
                onClick={() => {
                  const fileIds = Array.from(selectedFiles);
                  fileIds.forEach((fileId) => {
                    const link = document.createElement("a");
                    link.href = `/api/download/${fileId}`;
                    link.download = "";
                    document.body.appendChild(link);
                    link.click();
                    document.body.removeChild(link);
                  });
                  hideMenu();
                }}
              />

              {rightClickedFile?.id && selectedFiles.size <= 1 && (
                <ContextMenuItem
                  label="Rename"
                  glyph="pencil-line"
                  onClick={() => {
                    setIsRenamePopupOpen(true);
                    hideMenu();
                  }}
                />
              )}
              <ContextMenuItem
                label="Delete"
                glyph="trash-2"
                destructive
                onClick={() => {
                  setIsDeletePopupOpen(true);
                  hideMenu();
                }}
              />
            </ContextMenu>
          </div>
        )}
      </AnimatePresence>
      <AnimatePresence>
        {isDeletePopupOpen && (
          <DeleteFile
            files={
              Array.from(selectedFiles).some(
                (f) => f.id === rightClickedFile?.id,
              )
                ? Array.from(selectedFiles)
                : rightClickedFile
                  ? [rightClickedFile]
                  : []
            }
            fileName={rightClickedFile?.file_name}
            onClose={() => {
              setIsDeletePopupOpen(false);
            }}
          />
        )}

        {isRenamePopupOpen && rightClickedFile && (
          <RenameFile
            placeholder={rightClickedFile.file_name}
            fileId={rightClickedFile.id}
            onClose={() => {
              setIsRenamePopupOpen(false);
            }}
          />
        )}
      </AnimatePresence>
    </>
  );
}
