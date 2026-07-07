"use client";
import { FolderPlus, Upload } from "lucide-react";
import styles from "./Footer.module.scss";
import { useEffect, useRef, useState } from "react";
import CreateFolder from "./popups/CreateFolder";
import { AnimatePresence } from "motion/react";
import { createPortal } from "react-dom";
import Button from "../general/Button";

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
        <Button
          icon={<Upload size={14} strokeWidth={2.5} />}
          label="Upload File"
          variant="primary"
          onClick={() => inputRef.current?.click()}
        />
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
        <Button
          icon={<FolderPlus size={14} strokeWidth={2.5} />}
          label="Create Folder"
          variant="secondary"
          onClick={() => setIsFolderPopupOpen(true)}
        />
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
