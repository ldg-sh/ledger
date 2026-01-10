"use client";

import Popup from "./Popup";
import styles from "./RenameFile.module.scss";
import TextInput from "./TextInput";
import { cn } from "@/lib/util/class";
import { useState } from "react";
import { renameFile } from "@/lib/api/file";

interface RenameFileProps {
  onClose: () => void;
  placeholder?: string;
  fileId: string;
}

export default function RenameFile({ onClose, placeholder, fileId }: RenameFileProps) {
  const [value, setValue] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  function handleSubmit() {
    setIsLoading(true);

    renameFile(fileId, value).then(() => {
      let event = new CustomEvent("refresh-file-list", {
        detail: () => {
          setIsLoading(false);
          onClose();
        }
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
            <h1 className={styles.title}>Rename File</h1>
            <p className={styles.description}>
              Enter a new name for your file.
            </p>
          </div>
          <TextInput
            title="New File name"
            onChange={(newValue) => {
              setValue(newValue);
            }}
            onSubmit={handleSubmit}
            placeholder={placeholder || "Enter new file name"}
            select
          />
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
              disabled={!value}
              onClick={handleSubmit}
            >
              Submit
            </button>
          </div>
        </div>
      </Popup>
    </div>
  );
}
