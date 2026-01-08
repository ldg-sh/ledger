"use client";

import { useEffect, useRef, useState } from "react";
import styles from "./TransferWindow.module.scss";
import { createUpload, uploadPart } from "@/lib/api/file";
import { sha256_bytes } from "@/lib/util/hash";
import { pretifyFileSize } from "@/lib/util/file";

const CHUNK_SIZE = 5 * 1024 * 1024; // 5 MB
const MAX_CONCURRENT_UPLOADS = 3;

export default function TransferWindow() {
  const [progress, setProgress] = useState<ProgressMap>({});
  const [uploading, setUploading] = useState(false);
  const [isDragOver, setIsDragOver] = useState(false);

  const overlayRef = useRef<HTMLDivElement>(null);
  const taskQueue = useRef<UploadTask[]>([]);

  const handleDrop = async (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    const files = Array.from(e.dataTransfer.files);

    for (const file of files) {
      const totalChunks = Math.ceil(file.size / CHUNK_SIZE);

      let createRes = await createUpload(file.name, file.type, "");

      const fileId = createRes.file_id;
      console.log("Create upload response:", createRes);
      const uploadId = createRes.upload_id;

      setProgress((prev) => ({
        ...prev,
        [fileId]: {
          name: file.name,
          percent: 0,
          done: 0,
          total: totalChunks,
          fileId,
          uploadId,
          fileName: file.name,
          bytesUploaded: 0,
          totalBytes: file.size,
        },
      }));

      console.log("Created upload session:", createRes);

      for (let i = 0; i < totalChunks; i++) {
        const chunk = file.slice(i * CHUNK_SIZE, (i + 1) * CHUNK_SIZE);
        console.log(
          `Prepared chunk ${i + 1} of ${totalChunks} for file ${file.name}`
        );

        taskQueue.current.push({
          fileId,
          uploadId,
          fileName: file.name,
          chunkIndex: i,
          totalChunks,
          chunk,
        });
      }
    }

    if (!uploading) {
      await launchWorkers();
    } else {
      console.log("Upload already in progress, workers will pick up new tasks");
    }
  };

  const launchWorkers = async () => {
    setUploading(true);
    const workers = Array.from({ length: MAX_CONCURRENT_UPLOADS }, runWorker);
    await Promise.all(workers);
    setUploading(false);
  };

  const runWorker = async () => {
    while (taskQueue.current.length > 0) {
      const task = taskQueue.current.shift();
      if (task) {
        let uploadedAmount = await upload(task);

        setProgress((prev) => {
          const fileProg = prev[task.fileId];
          const done = fileProg.done + 1;
          const percent = Math.floor((done / fileProg.total) * 100);

          if (done >= fileProg.total) {
            setTimeout(() => {
              setProgress((prev) => {
                const newProgress = { ...prev };
                delete newProgress[task.fileId];

                return newProgress;
              });
            }, 2000);
          }

          return {
            ...prev,
            [task.fileId]: {
              ...fileProg,
              done,
              percent,
              fileId: task.fileId,
              uploadId: task.uploadId,
              fileName: task.fileName,
              bytesUploaded: fileProg.bytesUploaded + uploadedAmount,
              totalBytes: fileProg.totalBytes,
            },
          };
        });

        console.log(
          `Uploaded chunk ${task.chunkIndex + 1} of ${
            task.totalChunks
          } for file ${task.fileName}`
        );
      } else {
        break;
      }
    }
  };

  const upload = async (task: UploadTask) => {
    let data = task.chunk;
    let arrayBuffer = await data.arrayBuffer();
    let uint8Array = new Uint8Array(arrayBuffer);

    let checksum = await sha256_bytes(uint8Array);

    let uploadRes = await uploadPart(
      task.uploadId,
      "",
      task.fileId,
      checksum,
      task.chunkIndex + 1,
      task.totalChunks,
      uint8Array
    );

    return data.size;
  };

  const onDragOver = (e: React.DragEvent<HTMLDocument>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(true);
  };

  const onDragEnter = (e: React.DragEvent<HTMLDocument>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(true);
  };

  const onDragLeave = (e: React.DragEvent<HTMLDocument>) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(false);
  };

  useEffect(() => {
    const overlay = overlayRef.current;
    if (!overlay) return;

    document.addEventListener("dragover", (e) => {
      onDragOver(e as unknown as React.DragEvent<HTMLDocument>);
    });
    document.addEventListener("dragenter", (e) => {
      onDragEnter(e as unknown as React.DragEvent<HTMLDocument>);
    });
    document.addEventListener("dragleave", (e) => {
      onDragLeave(e as unknown as React.DragEvent<HTMLDocument>);
    });
    document.addEventListener("blur", () => {
      setIsDragOver(false);
    });
    document.addEventListener("drop", (e) => {
      handleDrop(e as unknown as React.DragEvent<HTMLDivElement>);
      onDragLeave(e as unknown as React.DragEvent<HTMLDocument>);
    });
  }, []);

  return (
    <>
      <div
        className={`${styles.dropOverlay} ${isDragOver ? styles.visible : ""}`}
        ref={overlayRef}
      >
        <div className={styles.borderBox}>
          <div className={styles.content}>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="80"
              height="80"
              viewBox="0 0 24 24"
              fill="none"
              stroke="var(--color-text-primary)"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <path className={styles.arrowPath} d="M12 13v8"></path>
              <path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"></path>
              <path className={styles.arrowPath} d="m8 17 4-4 4 4"></path>
            </svg>
            <div className={styles.text}>Drag and Drop</div>
            <div className={styles.subtext}>
              Files will be uploaded to the current directory
            </div>
          </div>
        </div>
      </div>

      <div className={styles.transferWindow}>
        <div className={styles.popupContent}>
          <div className={styles.header}>
            <h1 className={styles.title}>Active Transfers</h1>
            <p className={styles.subtitle}>
              {Object.values(progress).length} files uploading...
            </p>
          </div>
          
          <div className={styles.rows}>
            {Object.values(progress).map((fileProg) => (
              <div className={styles.fileProgress} key={fileProg.uploadId}>
                <div className={styles.fileInfo}>
                  <div className={styles.fileName}>{fileProg.fileName}</div>
                  <div className={styles.progressBar}>
                    <p className={styles.progressText}>{fileProg.percent}%</p>
                    <div className={styles.progressBars}>
                      <div className={styles.progressBackground}></div>
                      <div
                        className={styles.progressFill}
                        style={{ width: `${fileProg.percent}%` }}
                      ></div>
                    </div>
                  </div>
                </div>
                <div className={styles.progressNumbers}>
                  <p className={styles.bytesUploaded}>
                    {pretifyFileSize(fileProg.bytesUploaded)} /{" "}
                    {pretifyFileSize(fileProg.totalBytes)}
                  </p>
                  <p className={styles.chunksUploaded}>
                    {fileProg.done} / {fileProg.total}
                  </p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </>
  );
}
