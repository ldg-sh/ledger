"use client";

import Popup from "./Popup";
import styles from "./RenameFile.module.scss";
import TextInput from "./TextInput";
import { cn } from "@/lib/util/class";
import { useEffect, useRef, useState } from "react";
import { renameFile } from "@/lib/api/file";
import { useFile } from "@/context/FileExplorerContext";
import { useLoading } from "@/context/LoadingContext";

interface RenameFileProps {
  onClose: () => void;
  placeholder?: string;
  fileId: string;
}

export default function RenameFile({
  onClose,
  placeholder,
  fileId,
}: RenameFileProps) {
  const [value, setValue] = useState("");
  const submitButton = useRef<HTMLButtonElement>(null);
  const { setFileData } = useFile();
  const { setLoading } = useLoading();

  function handleSubmit() {
    setLoading(true);
    setFileData((prev) => {
      return {
        folders: prev.folders.map((folder) => {
          if (folder.id === fileId) {
            return { ...folder, file_name: value };
          } else {
            return folder;
          }
        }),
        files: prev.files.map((file) => {
          if (file.id === fileId) {
            return { ...file, file_name: value };
          } else {
            return file;
          }
        }),
      };
    });

    onClose();

    renameFile(fileId, value).then(() => {
      const event = new CustomEvent("refresh-file-list", {
        detail: () => {
          setLoading(false);
        },
      });

      window.dispatchEvent(event);
    });
  }

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.key === "Enter") {
      event.preventDefault();
      event.stopPropagation();
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
            <h1 className={styles.title}>Rename File</h1>
            <p className={styles.description}>
              Enter a new name for your file.
            </p>
          </div>
          <TextInput
            originalValue={placeholder}
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
              )}
              disabled={!value}
              onClick={handleSubmit}
              ref={submitButton}
            >
              Submit
            </button>
          </div>
        </div>
      </Popup>
    </div>
  );
}
