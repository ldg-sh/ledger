"use client";

import styles from "./Row.module.scss";
import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { pretifyFileSize } from "@/lib/util/file";
import { cn } from "@/lib/util/class";
import { ContextMenu } from "../general/menu/ContextMenu";
import { useCustomMenu } from "@/hooks/customMenu";
import ContextMenuItem from "../general/menu/ContextMenuItem";
import { AnimatePresence } from "motion/react";
import { useMemo, useState } from "react";
import RenameFile from "./popups/RenameFile";
import DeleteFile from "./popups/DeleteFile";
import GlyphButton from "../general/GlyphButton";
import { ListFileElement } from "@/lib/types/generated/ListFileElement";
import { useFile } from "@/context/FileExplorerContext";
import * as Icons from "lucide-react";
import { useLoading } from "@/context/LoadingContext";

interface RowProps {
  fileName: string;
  fileSize: number;
  fileType: string;
  fileId: string;
  createdAt?: string;
  folder?: boolean;
  file: ListFileElement;
  clickCallback?: (
    file: ListFileElement,
    selected: boolean,
    isShiftKey: boolean,
    isCommandKey: boolean,
  ) => void;
  selected?: boolean;
}

export default function Row({
  fileName,
  fileSize,
  fileType,
  fileId,
  createdAt = "",
  folder = false,
  selected = false,
  clickCallback,
  file,
}: RowProps) {
  const router = useRouter();
  const pathname = usePathname();
  const fileContext = useFile();
  const searchParams = useSearchParams();
  const loadingContext = useLoading();

  const { visible, position, showMenu, hideMenu } = useCustomMenu(fileId);

  const [isRenamePopupOpen, setIsRenamePopupOpen] = useState(false);
  const [isDeletePopupOpen, setIsDeletePopupOpen] = useState(false);

  const date = new Date(createdAt);

  const formattedDate = createdAt
    ? date.toLocaleString(undefined, {
        year: "numeric",
        month: "long",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      })
    : "";

  const ICON_MAP: Record<string, Icons.LucideIcon> = {
    folder: Icons.Folder,
    zip: Icons.FileArchive,
    image: Icons.Image,
    video: Icons.FileVideo,
    pdf: Icons.FileText,
    default: Icons.File,
  };

  const iconKey = folder ? "folder" : fileType.split("/")[0] || "default";
  const IconComponent = ICON_MAP[iconKey] || ICON_MAP.default;

  function handleDownload() {
    window.location.assign(`/api/download/${fileId}`);
  }

  return (
    <div className={styles.rowContainer}>
      <div className={styles.moreOptions} onClick={showMenu}>
        <GlyphButton
          glyph="ellipsis-vertical"
          size={16}
          fullSize={"30px"}
          color="var(--color-text-secondary)"
        />
      </div>
      <div
        className={cn(styles.row, selected && styles.selected)}
        data-file-id={fileId}
        data-file-name={fileName}
        data-context-boundary="true"
        onClick={(event) => {
          if (clickCallback) {
            const isShiftKey = event.shiftKey;
            const isCommandKey = event.metaKey || event.ctrlKey;

            clickCallback(file, selected, isShiftKey, isCommandKey);
          }
        }}
        onDoubleClick={() => {
          loadingContext.setLoading(true);
          const currentPath = fileContext.getPathFromUrl();

          if (folder) {
            fileContext.setBreadcrumbs([
              ...fileContext.breadcrumbs,
              {
                id: fileId,
                name: fileName,
              },
            ]);

            const params = new URLSearchParams(searchParams.toString());
            params.set("folder", fileId);

            router.push(`${pathname}?${params.toString()}`, { scroll: false });
          } else {
            window.open(`/preview/${currentPath}/${fileId}`, "_blank");
          }
        }}
      >
        <IconComponent
          size={16}
          strokeWidth={1.6}
          color={"var(--color-text-secondary)"}
          className={styles.rowElement}
        />

        <span
          className={cn(
            styles.fileName,
            styles.rowElement,
            folder && styles.folderLink,
          )}
        >
          {fileName}
        </span>
        <span className={cn(styles.fileSize, styles.rowElement)}>
          {fileSize !== 0 ? pretifyFileSize(fileSize) : "—"}
        </span>
        <span className={cn(styles.fileType, styles.rowElement)}>
          {fileType}
        </span>
        <span className={cn(styles.createdAt, styles.rowElement)}>
          {formattedDate !== "" ? formattedDate : "—"}
        </span>
      </div>
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
                    }),
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
                label="Copy Link"
                glyph="link"
                onClick={() => {
                  const link = `${window.location.origin}/api/download/${fileId}`;
                  navigator.clipboard.writeText(link);
                  hideMenu();
                }}
              />
              <ContextMenuItem
                label="Download"
                glyph="download"
                onClick={() => {
                  handleDownload();
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
            files={[file]}
            fileName={fileName}
            onClose={() => {
              setIsDeletePopupOpen(false);
            }}
          />
        )}
      </AnimatePresence>
    </div>
  );
}
