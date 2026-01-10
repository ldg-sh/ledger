"use client";

import { copyFile, copyMultipleFiles, listFiles } from "@/lib/api/file";
import Row from "./Row";
import { usePathname } from "next/navigation";
import { useEffect, useState } from "react";
import { extractPathFromUrl } from "@/lib/util/url";

export default function FileList() {
  let pathname = usePathname();
  const [data, setData] = useState<{ folders: Folder[]; files: File[] } | null>(
    null
  );

  let [selectedFiles, setSelectedFiles] = useState<Set<string>>(new Set());
  let [lastDeliberateClick, setLastDeliberateClick] = useState<string | null>(
    null
  );

  const copyFileIdsToClipboard = () => {
    if (selectedFiles.size > 0) {
      const fileIdsArray = Array.from(selectedFiles);

      const fileIdsString = fileIdsArray.join("\n");
      navigator.clipboard.writeText(fileIdsString).then(
        () => {},
        (err) => {
          console.error("Could not copy text: ", err);
        }
      );
    } else {
      if (lastDeliberateClick) {
        navigator.clipboard.writeText(lastDeliberateClick).then(
          () => {},
          (err) => {
            console.error("Could not copy text: ", err);
          }
        );
      }
    }
  };

  const copyFileIdToClipboard = (fileId: string) => {
    if (selectedFiles.size > 0) {
      const fileIdsArray = Array.from(selectedFiles);

      const fileIdsString = fileIdsArray.join("\n");
      navigator.clipboard.writeText(fileIdsString).then(
        () => {},
        (err) => {
          console.error("Could not copy text: ", err);
        }
      );
      
      return;
    }

    navigator.clipboard.writeText(fileId).then(
      () => {},
      (err) => {
        console.error("Could not copy text: ", err);
      }
    );
  };

  const pasteFileIdsFromClipboard = (ids: string[]) => {
    copyMultipleFiles(ids, extractPathFromUrl(pathname)).then(
      (file_ids: string[]) => {
        const event = new CustomEvent("refresh-file-list", {
          detail: () => {
            const newSelected = new Set(selectedFiles);
            newSelected.clear();

            file_ids.forEach((id) => {
              if (id) {
                newSelected.add(id);
              }
            });

            setSelectedFiles(newSelected);
          },
        });

        window.dispatchEvent(event);
      }
    );
  };

  const pasteFileIdFromClipboard = (id: string) => {
    copyFile(id.trim(), extractPathFromUrl(pathname)).then((fileId: string) => {
      const event = new CustomEvent("refresh-file-list", {
        detail: () => {
          setSelectedFiles((prevSelected) => {
            const newSelected = new Set(prevSelected);
            newSelected.clear();

            if (fileId) {
              newSelected.add(fileId);
            }

            return newSelected;
          });
        },
      });

      window.dispatchEvent(event);
    });
  };

  const handleSelectAll = () => {
    if (data) {
      const allFileIds: string[] = [];
      data.folders.forEach((folder) => {
        allFileIds.push(folder.folderName);
      });
      data.files.forEach((file) => {
        allFileIds.push(file.fileId);
      });
      setSelectedFiles(new Set(allFileIds));
    }
  };

  const handleSelectUpper = () => {
    if (data && lastDeliberateClick) {
      const allFileIds: string[] = [];
      data.folders.forEach((folder) => {
        allFileIds.push(folder.folderName);
      });
      data.files.forEach((file) => {
        allFileIds.push(file.fileId);
      });

      const lastIndex = allFileIds.indexOf(lastDeliberateClick);
      if (lastIndex > 0) {
        const newSelected = new Set(selectedFiles);
        newSelected.add(allFileIds[lastIndex - 1]);
        setSelectedFiles(newSelected);
        setLastDeliberateClick(allFileIds[lastIndex - 1]);
      }
    }
  };

  const handleSelectLower = () => {
    if (data && lastDeliberateClick) {
      const allFileIds: string[] = [];
      data.folders.forEach((folder) => {
        allFileIds.push(folder.folderName);
      });
      data.files.forEach((file) => {
        allFileIds.push(file.fileId);
      });

      const lastIndex = allFileIds.indexOf(lastDeliberateClick);
      if (lastIndex < allFileIds.length - 1) {
        const newSelected = new Set(selectedFiles);
        newSelected.add(allFileIds[lastIndex + 1]);
        setSelectedFiles(newSelected);
        setLastDeliberateClick(allFileIds[lastIndex + 1]);
      }
    }
  };

  const escape = () => {
    setSelectedFiles(new Set());
    setLastDeliberateClick(null);
  };

  async function getClipboardData() {
    try {
      const text = await navigator.clipboard.readText();

      return text
        .split("\n")
        .map((id) => id.trim())
        .filter((id) => id.length > 0);
    } catch (err) {
      console.error("Failed to read clipboard contents: ", err);
      return [];
    }
  }

  useEffect(() => {
    const handleKeyDown = async (event: KeyboardEvent) => {
      if ((event.metaKey || event.ctrlKey) && event.key === "a") {
        event.preventDefault();
        handleSelectAll();
      } else if (event.key === "Escape") {
        event.preventDefault();
        escape();
      } else if (event.key === "ArrowUp" && event.shiftKey) {
        event.preventDefault();
        handleSelectUpper();
      } else if (event.key === "ArrowDown" && event.shiftKey) {
        event.preventDefault();
        handleSelectLower();
      } else if ((event.metaKey || event.ctrlKey) && event.key === "c") {
        event.preventDefault();
        copyFileIdsToClipboard();
      } else if ((event.metaKey || event.ctrlKey) && event.key === "v") {
        event.preventDefault();

        let clipboard = await getClipboardData();

        if (clipboard.length === 1) {
          pasteFileIdFromClipboard(clipboard[0]);
        } else if (clipboard.length > 1) {
          pasteFileIdsFromClipboard(clipboard);
        }
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, [data, selectedFiles, lastDeliberateClick, pathname]);

  useEffect(() => {
    const handler = async (event: Event) => {
      if (event instanceof CustomEvent && event.detail) {
        const fileId = event.detail.fileId;
        copyFileIdToClipboard(fileId);
      } else {
        copyFileIdsToClipboard();
      }
      await refreshFileList(event);
    };

    document.addEventListener("copy-file-ids", handler);

    return () => {
      document.removeEventListener("copy-file-ids", handler);
    };
  }, [selectedFiles]);

  useEffect(() => {
    const handler = async (event: Event) => {
      let clipboard = await getClipboardData();

      if (clipboard.length === 1) {
        pasteFileIdFromClipboard(clipboard[0]);
      } else if (clipboard.length > 1) {
        pasteFileIdsFromClipboard(clipboard);
      }

      await refreshFileList(event);
    };

    document.addEventListener("paste-file-ids", handler);

    return () => {
      document.removeEventListener("paste-file-ids", handler);
    };
  }, [selectedFiles, pathname]);

  function handleRowClick(
    fileId: string,
    selected: boolean,
    isShiftKey: boolean,
    isCommandKey: boolean
  ) {
    setSelectedFiles((prevSelected) => {
      const newSelected = new Set(prevSelected);

      if (isCommandKey) {
        if (selected) {
          newSelected.delete(fileId);
        } else {
          newSelected.add(fileId);
        }
      } else if (isShiftKey && lastDeliberateClick) {
        newSelected.clear();

        let allFileIds: string[] = [];

        data?.folders.forEach((folder) => {
          allFileIds.push(folder.folderName);
        });
        data?.files.forEach((file) => {
          allFileIds.push(file.fileId);
        });

        let startIndex = allFileIds.indexOf(lastDeliberateClick);
        let endIndex = allFileIds.indexOf(fileId);

        if (startIndex > -1 && endIndex > -1) {
          let [from, to] =
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
  }

  useEffect(() => {
    loadData();

    window.addEventListener("refresh-file-list", async (event) => {
      await refreshFileList(event);
    });

    return () => {
      window.removeEventListener("refresh-file-list", async (event) => {
        await refreshFileList(event);
      });
    };
  }, [pathname]);

  async function refreshFileList(event: Event) {
    if (event instanceof CustomEvent && event.detail) {
      const onClose = event.detail;
      await loadData();

      if (onClose && typeof onClose === "function") {
        onClose();
      }
    } else {
      loadData();
    }
  }

  async function loadData() {
    let res = await listFiles(extractPathFromUrl(pathname));

    setData(res);
  }

  return (
    <div>
      {data?.folders.map((folder) => (
        <Row
          key={folder.folderName}
          fileName={folder.folderName}
          fileSize={folder.size}
          fileType="folder"
          folder={true}
          clickCallback={handleRowClick}
          selected={selectedFiles.has(folder.folderName)}
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
      <div style={{ height: "100px", width: "100%" }}></div>
    </div>
  );
}
