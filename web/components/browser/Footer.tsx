"use client";

import { FolderPlus, Upload } from "lucide-react";
import styles from "./Footer.module.scss";
import { useEffect, useRef, useState } from "react";
import { cn } from "@/lib/util/class";
import CreateFolder from "./popups/CreateFolder";
import { AnimatePresence } from "motion/react";
import { createPortal } from "react-dom";

export default function Footer() {
  const inputRef = useRef<HTMLInputElement>(null);

  const [isFolderPopupOpen, setIsFolderPopupOpen] = useState(false);

  const [mounted, setMounted] = useState(false);
  useEffect(() => {
    queueMicrotask(() => {
      setMounted(true);
    });
    return () => setMounted(false);
  }, []);

  return (
    <>
      <div className={styles.footerContainer}>
        <div
          className={cn(styles.uploadFile, styles.buttonComponent)}
          onClick={() => {
            inputRef.current?.click();
          }}
        >
          <Upload size={14} strokeWidth={2.5} />
          <span>Upload File</span>
        </div>
        <input
          type="file"
          style={{ display: "none" }}
          multiple
          ref={inputRef}
          onChange={(e) => {
            const event = new CustomEvent("trigger-upload", {
              detail: e.target.files,
            });

            window.dispatchEvent(event);

            (e.target as HTMLInputElement).value = "";
          }}
        />
        <div
          className={cn(
            styles.createFolder,
            styles.buttonComponent,
            styles.nonPrimaryElement,
          )}
          onClick={() => {
            setIsFolderPopupOpen(true);
          }}
        >
          <FolderPlus size={14} strokeWidth={2.5} />
          <span>Create Folder</span>
        </div>
      </div>

      {mounted &&
        createPortal(
          <AnimatePresence>
            {isFolderPopupOpen && (
              <CreateFolder onClose={() => setIsFolderPopupOpen(false)} />
            )}
          </AnimatePresence>,
          document.body,
        )}
    </>
  );
}
