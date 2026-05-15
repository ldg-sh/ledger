"use client";

import styles from "./FilePreview.module.scss";

export default function FilePreview({
  fileId,
  fileType,
}: {
  fileId: string;
  fileType?: string;
}) {
  return <div className={styles.container}></div>;
}
