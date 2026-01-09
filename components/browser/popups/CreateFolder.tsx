"use client";

import Popup from "./Popup";
import styles from "./CreateFolder.module.scss";
import TextInput from "./TextInput";
import { cn } from "@/lib/util/class";
import { useState } from "react";

interface CreateFolderProps {
  onCreate: (folderName: string) => void;
  onClose: () => void;
}

export default function CreateFolder({ onCreate, onClose }: CreateFolderProps) {
  const [value, setValue] = useState("");

  return (
    <div>
        <Popup
          onClosePopup={() => {
            onClose();
          }}
        >
          <div className={styles.createFolderContainer}>
            <div className={styles.text}>
              <h1 className={styles.title}>Create New Folder</h1>
              <p className={styles.description}>
                Enter a name for your new folder below.
              </p>
            </div>
            <TextInput
              title="Folder Name"
              onChange={(newValue) => {
                setValue(newValue);
              }}
              placeholder="/path/to/new/folder"
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
                className={cn(styles.submitButton, styles.actionButton)}
                disabled={!value}
                onClick={() => {
                  onCreate(value);
                }}
              >
                Submit
              </button>
            </div>
          </div>
        </Popup>
    </div>
  );
}
