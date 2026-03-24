"use client";

import { useEffect, useRef, useState } from "react";
import styles from "./TransferWindow.module.scss";
import { completeUpload, createUpload, uploadPart } from "@/lib/api/file";
import { sha256_bytes } from "@/lib/util/hash";
import { pretifyFileSize } from "@/lib/util/file";
import GlyphButton from "../general/GlyphButton";
import { cn } from "@/lib/util/class";
import { usePathname } from "next/navigation";
import { extractPathFromUrl } from "@/lib/util/url";
import { InitUploadResponse } from "@/lib/types/generated/InitUploadResponse";

const CHUNK_SIZE = 5 * 1024 * 1024; // 5 MB
const MAX_CONCURRENT_UPLOADS = 3;

export default function TransferWindow() {
  const [progress, setProgress] = useState<ProgressMap>({});
  const [uploading, setUploading] = useState(false);
  const [isDragOver, setIsDragOver] = useState(false);
  const [isExpanded, setIsExpanded] = useState(false);
  const [targetSize, setTargetSize] = useState(0);
  const [totalUploadedSize, setTotalUploadedSize] = useState(0);

  const path = usePathname();

  const overlayRef = useRef<HTMLDivElement>(null);
  const taskQueue = useRef<UploadTask[]>([]);

  useEffect(() => {
    const handleExternalUpload = async (e: Event) => {
      const files = (e as CustomEvent).detail as FileList;

      if (files.length > 0) {
        await handleDrop(files);
      }
    };

    window.addEventListener("trigger-upload", handleExternalUpload);

    return () =>
      window.removeEventListener("trigger-upload", handleExternalUpload);
  }, []);
  const handleDrop = async (e: React.DragEvent<HTMLDivElement> | FileList) => {
    const files: File[] =
      e instanceof FileList ? Array.from(e) : Array.from(e.dataTransfer.files);

    for (const file of files) {
      setTargetSize((prev) => prev + file.size);

      const totalChunks = Math.ceil(file.size / CHUNK_SIZE);
      const stateKey = Math.random().toString(36).substring(2, 15);

      setProgress((prev) => ({
        ...prev,
        [stateKey]: {
          name: file.name,
          percent: 0,
          done: 0,
          total: totalChunks,
          fileId: "",
          uploadUrl: "",
          fileName: file.name,
          bytesUploaded: 0,
          totalBytes: file.size,
          status: "Waiting...",
          stateKey,
        } as FileProgress,
      }));

      try {
        const createRes: InitUploadResponse = await createUpload(
          file.name,
          file.size,
          file.type,
          extractPathFromUrl(path),
        );

        setProgress((prev) => ({
          ...prev,
          [stateKey]: {
            ...prev[stateKey],
            fileId: createRes.file_id,
            uploadUrl: createRes.upload_url,
            uploadId: createRes.upload_id,
            status: "Uploading...",
          },
        }));

        for (let i = 0; i < totalChunks; i++) {
          taskQueue.current.push({
            fileId,
            uploadUrl: uploadUrl,
            fileName: file.name,
            chunkIndex: i,
            totalChunks,
            chunk: file.slice(i * CHUNK_SIZE, (i + 1) * CHUNK_SIZE),
            stateKey,
          });
        }

        if (!uploading) {
          launchWorkers();
        }
      } catch (err) {
        setProgress((prev) => ({
          ...prev,
          [stateKey]: { ...prev[stateKey], status: "Error" },
        }));
      }
    }
  };

  const launchWorkers = async () => {
  if (uploading) return;
  setUploading(true);
  
  const workerCount = Math.min(MAX_CONCURRENT_UPLOADS, taskQueue.current.length);
  
  const workers = Array.from({ length: workerCount }, runWorker);
  
  await Promise.all(workers);
  setUploading(false);
};

const runWorker = async () => {
  while (taskQueue.current.length > 0) {
    const task = taskQueue.current.shift();
    if (!task) break;

    try {
      const uploadedAmount = await upload(task);

      setProgress(async (prev) => {
        const fileProg = prev[task.stateKey];
        if (!fileProg) return prev;

        const newDone = fileProg.done + 1;
        const isComplete = newDone >= fileProg.total;

        if (isComplete) {
          await completeUpload(task.stateKey);
        }

        return {
          ...prev,
          [task.stateKey]: {
            ...fileProg,
            done: newDone,
            percent: Math.floor((newDone / fileProg.total) * 100),
            bytesUploaded: fileProg.bytesUploaded + uploadedAmount,
            status: isComplete ? "Completed" : "Uploading...",
          },
        };
      });

      setTotalUploadedSize((prev) => prev + uploadedAmount);
    } catch (error) {
      console.error("Worker task failed:", error);
    }
  }
};

  const upload = async (task: UploadTask) => {
    const uint8Array = new Uint8Array(await task.chunk.arrayBuffer());
    const checksum = await sha256_bytes(uint8Array);

    await uploadPart(
      task.uploadUrl,
      task.chunkIndex + 1,
      uint8Array,
      checksum,
    );

    return task.chunk.size;
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

    const dragOverHandler = (e: DragEvent) => {
      onDragOver(e as unknown as React.DragEvent<HTMLDocument>);
    };

    const onDragEnterHandler = (e: DragEvent) => {
      onDragEnter(e as unknown as React.DragEvent<HTMLDocument>);
    };

    const onDragLeaveHandler = (e: DragEvent) => {
      onDragLeave(e as unknown as React.DragEvent<HTMLDocument>);
    };

    const onDropHandler = (e: DragEvent) => {
      handleDrop(e as unknown as React.DragEvent<HTMLDivElement>);
      onDragLeave(e as unknown as React.DragEvent<HTMLDocument>);
    };

    document.addEventListener("dragover", dragOverHandler);
    document.addEventListener("dragenter", onDragEnterHandler);
    document.addEventListener("dragleave", onDragLeaveHandler);
    document.addEventListener("blur", () => {
      setIsDragOver(false);
    });
    document.addEventListener("drop", onDropHandler);

    return () => {
      document.removeEventListener("dragover", dragOverHandler);
      document.removeEventListener("dragenter", onDragEnterHandler);
      document.removeEventListener("dragleave", onDragLeaveHandler);
      document.removeEventListener("blur", () => {
        setIsDragOver(false);
      });
      document.removeEventListener("drop", onDropHandler);
    };
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
              stroke="var(--color-text-bold)"
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
            <div className={styles.left}>
              <h1 className={styles.title}>Active Transfers</h1>
              <div className={styles.subtitle}>
                {Object.values(progress).length === 0 ? (
                  "No active transfers"
                ) : (
                  <div>
                    {Object.values(progress).length} upload
                    {Object.values(progress).length !== 1 ? "s" : ""} in
                    progress{" "}
                    {Object.values(progress).length > 0
                      ? `- ${pretifyFileSize(
                          totalUploadedSize,
                        )} / ${pretifyFileSize(targetSize)}`
                      : ""}
                  </div>
                )}
                <div className={styles.progressBar}>
                  <div className={styles.progressBars}>
                    <div className={styles.progressBackground}></div>
                    <div
                      className={styles.progressFill}
                      style={{
                        width: `${
                          targetSize > 0
                            ? Math.floor((totalUploadedSize / targetSize) * 100)
                            : 0
                        }%`,
                      }}
                    ></div>
                  </div>
                </div>
              </div>
            </div>
            <div
              className={cn(styles.expandButton, isExpanded && styles.expanded)}
              onClick={() => {
                setIsExpanded(!isExpanded);
              }}
            >
              <GlyphButton glyph="chevron-down" size={24} rotate></GlyphButton>
            </div>
          </div>

          <div
            className={styles.rows}
            style={{
              maxHeight: isExpanded ? "300px" : "2px",
              minHeight: isExpanded ? "300px" : "2px",
              display: Object.keys(progress).length === 0 ? "flex" : "block",
            }}
          >
            {Object.keys(progress).length === 0 && (
              <div
                className={styles.subtitle}
                style={{ opacity: isExpanded ? 1 : 0 }}
              >
                <p>No active transfers</p>
              </div>
            )}
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
                        style={{
                          width: `${
                            progress[fileProg.stateKey].bytesUploaded > 0
                              ? (progress[fileProg.stateKey].bytesUploaded /
                                  progress[fileProg.stateKey].totalBytes) *
                                100
                              : 0
                          }%`,
                        }}
                      ></div>
                    </div>
                  </div>
                </div>
                <div className={styles.progressNumbers}>
                  {fileProg.status ? (
                    <p className={styles.bytesUploaded}>{fileProg.status}</p>
                  ) : (
                    <>
                      <p className={styles.bytesUploaded}>
                        {pretifyFileSize(fileProg.bytesUploaded)} /{" "}
                        {pretifyFileSize(fileProg.totalBytes)}
                      </p>
                      <p className={styles.chunksUploaded}>
                        {fileProg.done} / {fileProg.total}
                      </p>
                    </>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </>
  );
}
