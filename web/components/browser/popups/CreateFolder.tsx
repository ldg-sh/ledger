"use client";

import Popup from "./Popup";
import styles from "./CreateFolder.module.scss";
import TextInput from "./TextInput";
import { cn } from "@/lib/util/class";
import { useState } from "react";
import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { createDirectory } from "@/lib/api/directory";
import { useFile } from "@/context/FileExplorerContext";
import { DirectoryResponse } from "@/lib/types/generated/DirectoryResponse";
import { Breadcrumb } from "@/lib/types/generated/Breadcrumb";

interface CreateFolderProps {
  onClose: () => void;
}

export default function CreateFolder({ onClose }: CreateFolderProps) {
  const fileContext = useFile();

  const [value, setValue] = useState("");
  const [errorText, setErrorText] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const breadcrumbs = fileContext.breadcrumbs;

  function validateFolderName(name: string) {
    const folders = name.split("/").filter((part) => part.trim() !== "");
    const invalidChars = /[<>:"|?*]/;

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

  function collectBreadcrumbsToPath(breadcrumbs: Breadcrumb[]) {
    return breadcrumbs.reduce((path, folder) => {
      return path + "/" + folder.name;
    }, "");
  }

  function handleSubmit() {
    const validationError = validateFolderName(value);

    if (validationError) {
      setErrorText(validationError);

      setTimeout(() => {
        setErrorText("");
      }, 3000);

      return;
    }

    setIsLoading(true);

    createDirectory(fileContext.getPathFromUrl(), value).then(async (res) => {
      if (res.status === 409) {
        setIsLoading(false);
        setErrorText("A folder with that name already exists.");

        setTimeout(() => {
          setErrorText("");
        }, 3000);

        return;
      }

      setIsLoading(false);

      const responseData: DirectoryResponse = await res.json();

      fileContext.gotoPath(responseData.file_id);
      onClose();
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
              Folders are created from the current path.
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
                      collectBreadcrumbsToPath(breadcrumbs) +
                      "/" +
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
