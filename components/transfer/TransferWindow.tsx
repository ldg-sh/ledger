"use client";

import { useEffect, useRef, useState } from "react";
import styles from "./TransferWindow.module.scss";
import { createUpload, uploadPart } from "@/lib/api/file";
import { sha256_bytes } from "@/lib/util/hash";

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
      console.log("File size:", file.size);
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
        await upload(task);

        setProgress((prev) => {
          const fileProg = prev[task.fileId];
          const done = fileProg.done + 1;
          const percent = Math.floor((done / fileProg.total) * 100);

          return {
            ...prev,
            [task.fileId]: {
              ...fileProg,
              done,
              percent,
              fileId: task.fileId,
              uploadId: task.uploadId,
              fileName: task.fileName,
            },
          };
        });

        console.log(
          `Uploaded chunk ${task.chunkIndex + 1} of ${
            task.totalChunks
          } for file ${task.fileName}`
        );

        setTimeout(() => {
          setProgress((prev) => {
            const fileProg = prev[task.fileId];
            if (fileProg.done >= fileProg.total) {
              const newProgress = { ...prev };
              delete newProgress[task.fileId];

              return newProgress;
            } else {
              return prev;
            }
          });
        }, 2000);
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

    console.log("Upload part response:", uploadRes);
  };

  const onDragOver = (e: React.DragEvent<HTMLDocument>) => {
    console.log("Dragging over...");
    e.preventDefault();
    e.stopPropagation();
    setIsDragOver(true);
  };

  const onDragEnter = (e: React.DragEvent<HTMLDocument>) => {
    console.log("Drag entered the area");
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
          <h1 className={styles.title}>Active Transfers</h1>
          <p className={styles.subtitle}>
            {Object.values(progress).length} files uploading...
          </p>
          {Object.values(progress).map((fileProg) => (
            <div className={styles.fileProgress} key={fileProg.uploadId}>
              <div className={styles.fileName}>{fileProg.name}</div>
              <div className={styles.progressBarContainer}>
                <div
                  className={styles.progressBar}
                  style={{ width: `${fileProg.percent}%` }}
                ></div>
              </div>
              <div className={styles.progressText}>
                {fileProg.percent}% ({fileProg.done} / {fileProg.total} parts)
              </div>
            </div>
          ))}
        </div>
      </div>
    </>
  );
}
