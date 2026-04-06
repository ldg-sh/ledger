"use client";

import Popup from "./Popup";
import styles from "./DeleteFile.module.scss";
import { cn } from "@/lib/util/class";
import { useState } from "react";
import { deleteFiles } from "@/lib/api/file";
import { deleteDirectory } from "@/lib/api/directory";
import { File } from "@/lib/types/generated/File";
import { ListFileElement } from "@/lib/types/generated/ListFileElement";

interface DeleteFileProps {
  onClose: () => void;
  fileName?: string;
  files: ListFileElement[];
}

export default function DeleteFile({ onClose, files }: DeleteFileProps) {
  const [isLoading, setIsLoading] = useState(false);

  function handleSubmit() {
    setIsLoading(true);

    const directories = files.filter((file) => file.file_type === "directory");
    const regularFiles = files.filter((file) => file.file_type !== "directory");

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

    Promise.all(promises).then(() => {
      const event = new CustomEvent("refresh-file-list", {
        detail: () => {
          setIsLoading(false);
          onClose();
        },
      });

      window.dispatchEvent(event);
    });
  }

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
                isLoading && styles.loading,
              )}
              onClick={handleSubmit}
            >
              Delete
            </button>
          </div>
        </div>
      </Popup>
    </div>
  );
}
