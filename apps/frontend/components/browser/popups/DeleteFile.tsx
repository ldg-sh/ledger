"use client";

import Popup from "./Popup";
import styles from "./DeleteFile.module.scss";
import { cn } from "@/lib/util/class";
import { useState } from "react";
import { deleteFile } from "@/lib/api/file";

interface DeleteFileProps {
  onClose: () => void;
  fileIds: string[];
  fileName?: string;
}

export default function DeleteFile({ onClose, fileIds, fileName }: DeleteFileProps) {
  const [value, setValue] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  function handleSubmit() {
    setIsLoading(true);

    if (fileIds.length == 1) {
      deleteFile(fileIds[0]).then(() => {
        let event = new CustomEvent("refresh-file-list", {
          detail: () => {
            onClose();
            setIsLoading(false);
          },
        });

        window.dispatchEvent(event);
      });
    } else {
      Promise.all(fileIds.map((fileId) => deleteFile(fileId))).then(() => {
        let event = new CustomEvent("refresh-file-list", {
          detail: () => {
            onClose();
            setIsLoading(false);
          },
        });

        window.dispatchEvent(event);
      });
    }
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
            {fileIds.length > 1 ? (
            <p className={styles.description}>
                Are you sure you want to permanently delete <strong>{fileIds.length}</strong> files?
              </p>
            ) : (
              <p className={styles.description}>
                Are you sure you want to permanently delete <strong>{fileName}</strong>?
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
