"use client";

import Popup from "./Popup";
import styles from "./CreateFolder.module.scss";
import TextInput from "./TextInput";
import { cn } from "@/lib/util/class";
import { useState } from "react";
import { extractPathFromUrl } from "@/lib/util/url";
import { usePathname, useRouter } from "next/navigation";
import { createFolder } from "@/lib/api/create";

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

    createFolder(path, value).then(() => {
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
              Enter a name for your new folder below.
            </p>
          </div>
          <TextInput
            title="Folder Name"
            onChange={(newValue) => {
              setValue(newValue);
            }}
            onSubmit={handleSubmit}
            placeholder="path/to/new/folder"
            select
            hint={
              <>
                {value ? (
                  <p className={styles.hint}>
                    Your folder will be created {"at "}
                    <strong>
                      {" "}
                      {"home" +
                        (extractPathFromUrl(pathname) == ""
                          ? "/"
                          : "" + extractPathFromUrl(pathname)) +
                        value}
                    </strong>
                  </p>
                ) : (
                  <p className={styles.hint}>
                    Your folder will be created relative to the current path.
                  </p>
                )}
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
              Submit
            </button>
          </div>
        </div>
      </Popup>
    </div>
  );
}
