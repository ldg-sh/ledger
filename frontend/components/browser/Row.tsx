"use client";

import { DynamicIcon } from "lucide-react/dynamic";
import styles from "./Row.module.scss";
import { getFileIcon } from "@/lib/util/icon";
import { usePathname, useRouter } from "next/navigation";
import { extractPathFromUrl } from "@/lib/util/url";
import { pretifyFileSize } from "@/lib/util/file";
import { cn } from "@/lib/util/class";
import { ContextMenu } from "../general/menu/ContextMenu";
import { useCustomMenu } from "@/hooks/customMenu";
import ContextMenuItem from "../general/menu/ContextMenuItem";
import { AnimatePresence } from "motion/react";
import { useState } from "react";
import RenameFile from "./popups/RenameFile";
import DeleteFile from "./popups/DeleteFile";
import GlyphButton from "../general/GlyphButton";

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
  const { visible, position, showMenu, hideMenu } = useCustomMenu(folder ? fileName : fileId);

  const [isRenamePopupOpen, setIsRenamePopupOpen] = useState(false);
  const [isDeletePopupOpen, setIsDeletePopupOpen] = useState(false);

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
    <div className={styles.rowContainer}>
      <div className={styles.moreOptions} onClick={showMenu}>
        <GlyphButton
          glyph="ellipsis-vertical"
          size={16}
          fullSize={30}
          color="var(--color-text-secondary)"
        />
      </div>
      <div
        className={cn(styles.row, selected && styles.selected)}
        onContextMenu={showMenu}
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
          className={cn(
            styles.fileName,
            styles.rowElement,
            folder && styles.folderLink
          )}
        >
          {fileName}
        </span>
        <span className={cn(styles.fileSize, styles.rowElement)}>
          {pretifyFileSize(fileSize)}
        </span>
        <span className={cn(styles.fileType, styles.rowElement)}>
          {fileType}
        </span>
        <span className={cn(styles.createdAt, styles.rowElement)}>
          {formattedDate}
        </span>
        <AnimatePresence>
          {visible && (
            <div>
              <ContextMenu x={position.x} y={position.y}>
                <ContextMenuItem
                  label="Copy"
                  glyph="copy"
                  hotkey="CtrlC"
                  onClick={() => {
                    document.dispatchEvent(
                      new CustomEvent("copy-file-ids", {
                        detail: {
                          fileId: fileId,
                        },
                      })
                    );
                    hideMenu();
                  }}
                />
                <ContextMenuItem
                  label="Paste"
                  glyph="clipboard-paste"
                  hotkey="CtrlV"
                  onClick={() => {
                    document.dispatchEvent(new CustomEvent("paste-file-ids"));

                    hideMenu();
                  }}
                />
                <ContextMenuItem
                  label="Rename"
                  glyph="pencil-line"
                  onClick={() => {
                    setIsRenamePopupOpen(true);
                    hideMenu();
                  }}
                />
                <ContextMenuItem
                  label="Delete"
                  glyph="trash-2"
                  destructive
                  onClick={() => {
                    setIsDeletePopupOpen(true);
                    hideMenu();
                  }}
                />
              </ContextMenu>
            </div>
          )}
        </AnimatePresence>

        <AnimatePresence>
          {isRenamePopupOpen && (
            <RenameFile
              placeholder={fileName}
              fileId={fileId}
              onClose={() => {
                setIsRenamePopupOpen(false);
              }}
            />
          )}
          {isDeletePopupOpen && (
            <DeleteFile
              fileId={fileId}
              onClose={() => {
                setIsDeletePopupOpen(false);
              }}
            />
          )}
        </AnimatePresence>
      </div>
    </div>
  );
}
