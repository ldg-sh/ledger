"use client";

import Popup from "./Popup";
import styles from "./DeleteFile.module.scss";
import { cn } from "@/lib/util/class";
import { useRef, useEffect } from "react";
import { deleteFiles } from "@/lib/api/file";
import { deleteDirectory } from "@/lib/api/directory";
import { ListFileElement } from "@/lib/types/generated/ListFileElement";
import { useFile } from "@/context/FileExplorerContext";
import { useLoading } from "@/context/LoadingContext";

interface DeleteFileProps {
  onClose: () => void;
  fileName?: string;
  files: ListFileElement[];
}

export default function DeleteFile({ onClose, files }: DeleteFileProps) {
  const submitButton = useRef<HTMLButtonElement>(null);
  const { setFileData } = useFile();
  const { setLoading } = useLoading();

  function handleSubmit() {
    setLoading(true);

    const directories = files.filter((file) => file.is_directory);
    const regularFiles = files.filter((file) => !file.is_directory);

    const promises = [];

    if (directories.length > 0) {
      for (const dir of directories) {
        const basePath = dir.path.endsWith("/")
          ? dir.path.slice(0, -1)
          : dir.path;
        promises.push(deleteDirectory(basePath + "/" + dir.file_name, dir.id));
      }
    }

    if (regularFiles.length == 1) {
      promises.push(deleteFiles([regularFiles[0].id]));
    } else if (regularFiles.length > 1) {
      promises.push(deleteFiles(regularFiles.map((f) => f.id)));
    }

    setFileData((prev) => {
      return {
        files: prev?.files.filter((f) => !regularFiles.map((file) => file.id).includes(f.id)),
        folders: prev?.folders.filter((f) => !directories.map((dir) => dir.id).includes(f.id)),
      }
    });

    Promise.all(promises).then(() => {
      const event = new CustomEvent("refresh-file-list", {
        detail: () => {
          setLoading(false);
        },
      });

      window.dispatchEvent(event);
    });

    onClose();
  }

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.key === "Enter") {
      submitButton.current?.click();
    }
  };

  useEffect(() => {
    window.addEventListener("keydown", handleKeyDown);
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, []);


  return (
    <div>
      <Popup
        onClosePopup={() => {
          onClose();
        }}
      >
        <div className={styles.renameFileContainer}>
          <div className={styles.text}>
            <h1 className={styles.title}>Confirm Deletion</h1>
            {files.length > 1 ? (
              <p className={styles.description}>
                Are you sure you want to permanently delete{" "}
                <strong>{files.length}</strong> files?
              </p>
            ) : (
              <p className={styles.description}>
                Are you sure you want to permanently delete{" "}
                <strong>{files[0].file_name}</strong>?
              </p>
            )}
          </div>
          <div className={styles.actions}>
            <button
              className={cn(styles.cancelButton, styles.actionButton)}
              onClick={() => {
                onClose();
              }}
            >
              Cancel
            </button>
            <button
              className={cn(
                styles.submitButton,
                styles.actionButton,
              )}
              onClick={handleSubmit}
              ref={submitButton}
            >
              Delete
            </button>
          </div>
        </div>
      </Popup>
    </div>
  );
}
