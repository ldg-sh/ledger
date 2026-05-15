"use client";

import { useFile } from "@/context/FileExplorerContext";
import { setGlobalLoading } from "@/context/LoadingContext";
import { useSort } from "@/context/SortContext";
import { useUser } from "@/context/UserContext";
import { useCustomMenu } from "@/hooks/customMenu";
import { copyFiles, getShareLink, listFiles, moveFiles } from "@/lib/api/file";
import { ListFileElement } from "@/lib/types/generated/ListFileElement";
import { handleClientDownload } from "@/lib/util/download";
import { AnimatePresence } from "motion/react";
import { useCallback, useEffect, useRef, useState } from "react";
import { ContextMenu } from "../general/menu/ContextMenu";
import ContextMenuItem from "../general/menu/ContextMenuItem";
import styles from "./FileList.module.scss";
import DeleteFile from "./popups/DeleteFile";
import RenameFile from "./popups/RenameFile";
import Row from "./Row";

interface FileListProps {
  parentContainerRef?: React.RefObject<HTMLDivElement>;
}

const CHUNK_SIZE = 150;
const THROTTLE_MS = 500;

export default function FileList({ parentContainerRef }: FileListProps) {
  const { sort } = useSort();
  const fileContext = useFile();

  const lastRefreshTime = useRef<number>(0);

  const [selectedFiles, setSelectedFiles] = useState<Set<ListFileElement>>(
    new Set(),
  );
  const [lastDeliberateClick, setLastDeliberateClick] =
    useState<ListFileElement | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [currentOffset, setCurrentOffset] = useState(0);
  const [hasMore, setHasMore] = useState(true);
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [copyLinkLoading, setCopyLinkLoading] = useState(false);

  const [rightClickedFile, setRightClickedFile] =
    useState<ListFileElement | null>(null);

  const [isRenamePopupOpen, setIsRenamePopupOpen] = useState(false);
  const [isDeletePopupOpen, setIsDeletePopupOpen] = useState(false);

  const { visible, position, showMenu, hideMenu } =
    useCustomMenu("file-list-menu");

  const { user, loading: authLoading } = useUser();

  const getAllFiles = useCallback((): ListFileElement[] => {
    if (!fileContext.fileData) return [];
    return [...fileContext.fileData.folders, ...fileContext.fileData.files];
  }, [fileContext.fileData]);

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
      const fileIdsString = Array.from(selectedFiles)
        .map((f) => f.id)
        .join("\n");
      await writeToClipboard(fileIdsString);
    } else if (lastDeliberateClick) {
      await writeToClipboard(lastDeliberateClick.id);
    }
  }, [selectedFiles, lastDeliberateClick]);

  const copyFileIdToClipboard = useCallback(async (fileId: string) => {
    await writeToClipboard(fileId);
  }, []);

  const pasteFileIdsFromClipboard = useCallback(
    async (ids: string[]) => {
      const fileIdsToCopy: string[] = [];

      ids.forEach((id) => {
        const file = fileContext.fileData?.files.find((file) => file.id === id);
        if (file) {
          fileIdsToCopy.push(id);
        }
      });

      const fileIds = await copyFiles(
        fileIdsToCopy,
        fileContext.currentFolderId,
      );

      const event = new CustomEvent("refresh-file-list", {
        detail: () => {
          const newlyCopiedFiles = fileIds
            .map((id) =>
              fileContext.fileData?.files.find((file) => file.id === id),
            )
            .filter((f): f is ListFileElement => !!f);

          setSelectedFiles(new Set(newlyCopiedFiles));
        },
      });
      window.dispatchEvent(event);
    },
    [fileContext],
  );

  const pasteFileIdFromClipboard = useCallback(
    async (id: string) => {
      const file = fileContext.fileData?.files.find((file) => file.id === id);

      let newId = "";

      if (file) {
        newId = (await copyFiles([id.trim()], fileContext.currentFolderId))[0];
      }

      const event = new CustomEvent("refresh-file-list", {
        detail: () => {
          const newFile = fileContext.fileData?.files.find(
            (f) => f.id === newId,
          );

          if (newFile) {
            setSelectedFiles(new Set([newFile]));
          } else {
            setSelectedFiles(new Set());
          }
        },
      });
      window.dispatchEvent(event);
    },
    [fileContext],
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
          if (selected) {
            newSelected.delete(file);
          } else {
            newSelected.add(file);
          }
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

    setSelectedFiles(new Set());

    setIsLoading(true);
    setGlobalLoading(true);
    try {
      const res = await listFiles(
        fileContext.currentFolderId,
        sort,
        0,
        CHUNK_SIZE,
        fileContext.searchQuery,
      );

      if (!res.hasMore) {
        setHasMore(false);
      } else {
        setHasMore(true);
      }

      fileContext.setFileData(res);

      fileContext.setBreadcrumbs(res.breadcrumbs);
    } finally {
      setIsLoading(false);
      setGlobalLoading(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [
    authLoading,
    user,
    sort,
    fileContext.setBreadcrumbs,
    fileContext.currentFolderId,
    fileContext.searchQuery,
  ]);

  const loadMoreData = useCallback(async () => {
    if (authLoading || !hasMore || isLoading) return;

    setIsLoading(true);
    setGlobalLoading(true);

    const res = await listFiles(
      fileContext.currentFolderId,
      sort,
      currentOffset,
      CHUNK_SIZE,
      fileContext.searchQuery,
    );

    if (!res.hasMore) {
      setHasMore(false);
    } else {
      setHasMore(true);
    }

    fileContext.setFileData((prevData) => {
      if (!prevData) return res;

      res.files = res.files.filter(
        (newFile) =>
          !prevData.files.some(
            (existingFile) => existingFile.id === newFile.id,
          ) && newFile.upload_completed,
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
        breadcrumbs: res.breadcrumbs,
      };
    });
    setIsLoading(false);
    setGlobalLoading(false);
  }, [fileContext, sort, currentOffset, authLoading, hasMore, isLoading]);

  useEffect(() => {
    setCurrentOffset(0);

    if (!authLoading && user) {
      loadData();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [loadData, authLoading]);

  const refreshFileList = useCallback(
    async (event: Event) => {
      const now = Date.now();

      if (isRefreshing || now - lastRefreshTime.current < THROTTLE_MS) {
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
            fileContext.fileData?.files.find((f) => f.id === event.detail) ||
            fileContext.fileData?.folders.find((f) => f.id === event.detail);

          if (selectedFiles.size === 0 && file) {
            setSelectedFiles(new Set([file]));
          }
        }
      } finally {
        setIsRefreshing(false);
      }
    },
    [isRefreshing, loadData, fileContext.fileData, selectedFiles.size],
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

  const handleMoveFiles = useCallback(
    async (sourceFiles: ListFileElement[], targetFolderId: string) => {
      const validFiles = sourceFiles.filter((f) => f.id !== targetFolderId);
      if (validFiles.length === 0) return;

      try {
        setGlobalLoading(true);
        await moveFiles(
          validFiles.map((f) => f.id),
          targetFolderId,
        );

        window.dispatchEvent(new CustomEvent("refresh-file-list"));
      } catch (err) {
        console.error("Failed to move files:", err);
      } finally {
        setGlobalLoading(false);
      }
    },
    [],
  );

  useEffect(() => {
    if (currentOffset > 0) {
      loadMoreData();
    }
  }, [currentOffset, loadMoreData]);

  const onDragStart = (e: React.DragEvent, file: ListFileElement) => {
    let filesToMove: ListFileElement[];

    if (selectedFiles.has(file)) {
      filesToMove = Array.from(selectedFiles);
    } else {
      filesToMove = [file];
      setSelectedFiles(new Set([file]));
    }

    const fileIds = filesToMove.map((f) => f.id).join(",");
    e.dataTransfer.setData("text/plain", fileIds);

    const dragPreview = document.createElement("div");
    dragPreview.style.position = "absolute";
    dragPreview.style.top = "-1000px";
    dragPreview.style.display = "flex";
    dragPreview.style.alignItems = "center";
    dragPreview.style.justifyContent = "center";
    dragPreview.style.padding = "8px 12px";
    dragPreview.style.background = "var(--color-background-hover)";
    dragPreview.style.border = "var(--border-style)";
    dragPreview.style.borderRadius = "8px";
    dragPreview.style.color = "var(--color-text-primary)";
    dragPreview.style.fontSize = "13px";
    dragPreview.style.fontWeight = "500";
    dragPreview.style.pointerEvents = "none";
    dragPreview.style.whiteSpace = "nowrap";
    dragPreview.style.zIndex = "10000";
    dragPreview.style.gap = "4px";

    const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("xmlns", "http://www.w3.org/2000/svg");
    svg.setAttribute("width", "14");
    svg.setAttribute("height", "14");
    svg.setAttribute("viewBox", "0 0 24 24");
    svg.setAttribute("fill", "none");
    svg.setAttribute("stroke", "currentColor");
    svg.setAttribute("stroke-width", "2.5");
    svg.setAttribute("stroke-linecap", "round");
    svg.setAttribute("stroke-linejoin", "round");

    const path1 = document.createElementNS(
      "http://www.w3.org/2000/svg",
      "path",
    );
    path1.setAttribute("d", "m15 10 5 5-5 5");
    svg.appendChild(path1);

    const path2 = document.createElementNS(
      "http://www.w3.org/2000/svg",
      "path",
    );
    path2.setAttribute("d", "M4 4v7a4 4 0 0 0 4 4h12");
    svg.appendChild(path2);

    dragPreview.appendChild(svg);

    const icon = document.createElement("span");
    dragPreview.appendChild(icon);

    const text = document.createElement("span");
    text.innerText =
      filesToMove.length > 1
        ? `Moving ${filesToMove.length} items`
        : `Moving ${file.file_name}`;
    dragPreview.appendChild(text);

    document.body.appendChild(dragPreview);
    e.dataTransfer.setDragImage(dragPreview, 20, 20);

    setTimeout(() => {
      document.body.removeChild(dragPreview);
    }, 0);
  };

  return (
    <>
      <div
        className={styles.rowsContainer}
        onContextMenu={(event) => {
          const target = event.target as HTMLElement;

          event.preventDefault();

          const rowElement = target.closest("[data-context-boundary]");

          const fileId = rowElement?.getAttribute("data-file-id");
          const fileName = rowElement?.getAttribute("data-file-name");

          const file =
            fileContext.fileData?.files.find((f) => f.id === fileId) ||
            fileContext.fileData?.folders.find((f) => f.id === fileId);

          if (fileId && fileName) {
            setRightClickedFile(file || null);
            setLastDeliberateClick(file || null);
          } else {
            setLastDeliberateClick(null);
          }

          showMenu(event);
        }}
      >
        {fileContext.fileData?.folders.map((folder) => (
          <Row
            key={folder.id}
            fileId={folder.id}
            fileName={folder.file_name}
            createdAt={folder.created_at}
            fileSize={folder.file_size as unknown as number}
            fileType="folder"
            folder={true}
            clickCallback={handleRowClick}
            selected={selectedFiles.has(folder)}
            draggable={true}
            dropable={true}
            onDragStart={(e) => onDragStart(e, folder)}
            onDrop={(e) => {
              e.preventDefault();
              handleMoveFiles(Array.from(selectedFiles), folder.id);
            }}
            file={folder}
          />
        ))}
        {fileContext.fileData?.files.map((file) => (
          <Row
            key={file.id}
            fileName={file.file_name}
            fileSize={file.file_size as unknown as number}
            fileType={file.file_type}
            fileId={file.id}
            createdAt={file.created_at}
            clickCallback={handleRowClick}
            selected={selectedFiles.has(file)}
            draggable={true}
            dropable={false}
            onDragStart={(e) => onDragStart(e, file)}
            onDrop={(e) => {
              e.preventDefault();
              handleMoveFiles(Array.from(selectedFiles), file.id);
            }}
            file={file}
          />
        ))}
        {!isLoading &&
        fileContext.fileData &&
        fileContext.fileData.files.length === 0 &&
        fileContext.fileData.folders.length === 0 ? (
          <div className={styles.emptyState}>
            <svg
              className={styles.svg}
              width="35"
              height="35"
              viewBox="0 0 514 514"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M308.906 43.0439H357.015V156.985H465.892V200.029H514V357.015H205.094V514H43.0439V465.892H0V308.906H156.985V351.951H200.029V308.906H156.985V0H308.906V43.0439ZM156.985 465.892H48.1084V508.936H200.029V357.015H156.985V46.892ZM465.892 308.906H205.094V351.951H508.936V205.094H465.892V308.906ZM308.906 156.985H351.951V48.1084H308.906V156.985Z"
                fill="var(--color-text-secondary)"
              />
            </svg>
            {fileContext.searchQuery ? (
              <span className={styles.searchQuery}>
                No results found for "{fileContext.searchQuery}".
              </span>
            ) : (
              "This folder is empty."
            )}
          </div>
        ) : (
          <div className={styles.spacer} />
        )}
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
              {(selectedFiles.size === 1 ||
                (rightClickedFile != null && selectedFiles.size <= 1)) && (
                <ContextMenuItem
                  label="Copy Shareable Link"
                  glyph="link"
                  isLoading={copyLinkLoading}
                  onClick={async () => {
                    const targetFile =
                      rightClickedFile ?? selectedFiles.values().next().value;

                    if (targetFile?.id) {
                      try {
                        setCopyLinkLoading(true);
                        const url = await getShareLink(
                          targetFile.id,
                          targetFile.file_name,
                          targetFile.file_type,
                          targetFile.file_size,
                          targetFile.created_at,
                        );
                        await navigator.clipboard.writeText(url);
                      } catch (err) {
                        console.error("Failed to copy share link:", err);
                      }
                    }

                    hideMenu();
                    setCopyLinkLoading(false);
                  }}
                />
              )}
              <ContextMenuItem
                label="Download"
                glyph="download"
                onClick={() => {
                  const files = Array.from(selectedFiles).some(
                    (f) => f.id === rightClickedFile?.id,
                  )
                    ? Array.from(selectedFiles)
                    : rightClickedFile
                      ? [rightClickedFile]
                      : [];

                  if (files.length == 1 && files[0].file_type !== "directory") {
                    handleClientDownload([files[0].id], files[0].file_name);
                  } else {
                    handleClientDownload(files.map((f) => f.id));
                  }
                  hideMenu();
                }}
              />

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
