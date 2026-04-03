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
  const [errorText, setErrorText] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  function validateFolderName(name: string) {
    let folders = name.split("/").filter((part) => part.trim() !== "");
    let invalidChars = /[<>:"|?*]/;

    if (invalidChars.test(name)) {
      return "Folder name contains invalid characters.";
    }

    if (folders.some((folder) => folder.length === 0)) {
      return "Folder name cannot contain empty parts.";
    }

    if (folders.length == 0) {
      return "Folder name cannot be empty.";
    }

    if (name.length === 0) {
      return "Folder name cannot be empty.";
    }
    if (name.length > 255) {
      return "Folder name cannot exceed 255 characters.";
    }
    return null;
  }

  function handleSubmit() {
    let path = extractPathFromUrl(pathname);
    let validationError = validateFolderName(value);

    if (validationError) {
      setErrorText(validationError);

      setTimeout(() => {
        setErrorText("");
      }, 3000);

      return;
    }

    setIsLoading(true);

    createDirectory(path, value).then((res) => {
      if (res.status === 409) {
        setIsLoading(false);
        setErrorText("A folder with that name already exists.");

        setTimeout(() => {
          setErrorText("");
        }, 3000);

        return;
      }

      setIsLoading(false);

      router.push(pathname + (pathname.endsWith("/") ? "" : "/") + value);
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
            errorHint={true}
            select
            // Thank you, Dom, for this hint design.
            hint={
              errorText ? (
                <p className={cn(styles.hint, styles.error)}>{errorText}</p>
              ) : (
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
              )
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
                isLoading && styles.loading,
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
