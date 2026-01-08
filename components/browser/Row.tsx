"use client";

import { Square } from "lucide-react";
import { DynamicIcon } from "lucide-react/dynamic";
import styles from "./Row.module.scss";
import { getFileIcon } from "@/lib/util/icon";
import { usePathname, useRouter } from "next/navigation";
import { extractPathFromUrl } from "@/lib/util/url";
import { pretifyFileSize } from "@/lib/util/file";

interface RowProps {
  fileName: string;
  fileSize: number;
  fileType: string;
  fileId?: string;
  createdAt?: string;
  folder?: boolean;
  clickCallback?: (
    fileId: string,
    selected: boolean,
    isShiftKey: boolean,
    isCommandKey: boolean
  ) => void;
  selected?: boolean;
}

export default function Row({
  fileName,
  fileSize,
  fileType,
  fileId = "",
  createdAt = "",
  folder = false,
  selected = false,
  clickCallback,
}: RowProps) {
  let router = useRouter();
  let pathname = usePathname();

  let date = new Date(createdAt);
  let formattedDate = createdAt
    ? date.toLocaleString(undefined, {
        year: "numeric",
        month: "long",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      })
    : "";

  return (
    <div
      className={styles.row + (selected ? " " + styles.selected : "")}
      onClick={(event) => {
        if (clickCallback) {
          let isShiftKey = event.shiftKey;
          let isCommandKey = event.metaKey || event.ctrlKey;

          let newFileId = fileId;

          if (folder) {
            newFileId = fileName;
          }

          clickCallback(newFileId, selected, isShiftKey, isCommandKey);
        }
      }}
      onDoubleClick={() => {
        let currentPath = extractPathFromUrl(pathname);

        if (folder) {
          router.push(
            `/dashboard/${
              currentPath === "/" ? "" : currentPath + "/"
            }${fileName}`
          );
        } else {
          window.open(`/preview/${currentPath}/${fileId}`, "_blank");
        }
      }}
    >
      <Square
        size={16}
        strokeWidth={1.6}
        color={"var(--color-text-secondary)"}
        className={styles.rowElement}
      />
      {folder ? (
        <DynamicIcon
          name={"folder"}
          size={16}
          strokeWidth={1.6}
          color={"var(--color-text-secondary)"}
          className={styles.rowElement}
        />
      ) : (
        <DynamicIcon
          name={getFileIcon(fileType) as any}
          size={16}
          strokeWidth={1.6}
          color={"var(--color-text-secondary)"}
          className={styles.rowElement}
        />
      )}
      <span
        className={
          styles.fileName +
          " " +
          styles.rowElement +
          (folder ? " " + styles.folderLink : "")
        }
      >
        {fileName}
      </span>
      <span className={styles.fileSize + " " + styles.rowElement}>
        {pretifyFileSize(fileSize)}
      </span>
      <span className={styles.fileType + " " + styles.rowElement}>
        {fileType}
      </span>
      <span className={styles.createdAt + " " + styles.rowElement}>
        {formattedDate}
      </span>
    </div>
  );
}
