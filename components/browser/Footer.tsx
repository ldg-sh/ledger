"use client";

import { FolderPlus, Upload } from "lucide-react";
import styles from "./Footer.module.scss";
import { useRef } from "react";

export default function Footer() {
  let inputRef = useRef<HTMLInputElement>(null);

  return (
    <div className={styles.footerContainer}>
      <div
        className={styles.uploadFile + " " + styles.buttonComponent}
        onClick={() => {
          inputRef.current?.click();
        }}
      >
        <Upload size={16} strokeWidth={2.5} />
        <span>Upload File</span>
      </div>
      <input
        type="file"
        style={{ display: "none" }}
        multiple
        ref={inputRef}
        onChange={(e) => {
          console.log(e.target.files);
          const event = new CustomEvent("trigger-upload", {
            detail: e.target.files,
          });
          
          window.dispatchEvent(event);
        }}
      />
      <div
        className={
          styles.createFolder +
          " " +
          styles.buttonComponent +
          " " +
          styles.nonPrimaryElement
        }
      >
        <FolderPlus size={16} strokeWidth={2.5} />
        <span>Create Folder</span>
      </div>
    </div>
  );
}
