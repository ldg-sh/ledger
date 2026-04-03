"use client";

import Popup from "./Popup";
import styles from "./CreateFolder.module.scss";
import TextInput from "./TextInput";
import { cn } from "@/lib/util/class";
import { useState } from "react";
import { extractPathFromUrl } from "@/lib/util/url";
import { usePathname, useRouter } from "next/navigation";
import { createDirectory } from "@/lib/api/directory";

interface CreateFolderProps {
  onClose: () => void;
}

export default function CreateFolder({ onClose }: CreateFolderProps) {
  let pathname = usePathname();
  let router = useRouter();

  const [value, setValue] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  function handleSubmit() {
    let path = extractPathFromUrl(pathname);
    setIsLoading(true);

    createDirectory(path, value).then(() => {
      setIsLoading(false);

      router.push(
        pathname + (pathname.endsWith("/") ? "" : "/") + value
      );
    });
  }

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
              Folders are created relative to the current path.
            </p>
          </div>
          <TextInput
            onChange={(newValue) => {
              setValue(newValue);
            }}
            onSubmit={handleSubmit}
            placeholder="path/to/new/folder"
            select
            // Thank you, Dom, for this hint design.
            hint={
              <>
                  <p className={styles.hint}>
                    Your folder will be created at
                    <strong>
                      {" home" +
                        (extractPathFromUrl(pathname) == ""
                          ? "/"
                          : "" + extractPathFromUrl(pathname)) +
                        value}
                    </strong>
                  </p>
              </>
            }
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
              Create
            </button>
          </div>
        </div>
      </Popup>
    </div>
  );
}
