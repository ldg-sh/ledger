"use client";

import styles from "./FilePreview.module.scss";

export default function FilePreview({ fileId, fileType }: { fileId: string; fileType?: string }) {
  const previewUrl = `/api/download/${fileId}?preview=true`;
  const isImage = fileType?.startsWith("image/");

  return (
    <div className={styles.container}>
      {isImage ? (
        <img 
          src={previewUrl} 
          alt="Preview" 
          className={styles.previewImage} 
        />
      ) : (
        <iframe
          src={previewUrl}
          title="File Preview"
          className={styles.iframe}
        />
      )}
    </div>
  );
}