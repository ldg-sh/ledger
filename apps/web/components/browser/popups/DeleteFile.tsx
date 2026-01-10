"use client";

import Popup from "./Popup";
import styles from "./DeleteFile.module.scss";
import { cn } from "@/lib/util/class";
import { useState } from "react";
import { deleteFile } from "@/lib/api/file";

interface DeleteFileProps {
  onClose: () => void;
  fileId: string;
}

export default function DeleteFile({ onClose, fileId }: DeleteFileProps) {
  const [value, setValue] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  function handleSubmit() {
    setIsLoading(true);

    deleteFile(fileId).then(() => {
      let event = new CustomEvent("refresh-file-list", {
        detail: () => {
          onClose();
          setIsLoading(false);
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
            <p className={styles.description}>
              Are you sure you want to permanently delete this file?
            </p>
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
                isLoading && styles.loading
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
