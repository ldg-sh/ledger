"use client";

import { useEffect, useRef, useState } from "react";
import styles from "./TransferWindow.module.scss";
import { completeUpload, createUpload, uploadPart } from "@/lib/api/file";
import { pretifyFileSize } from "@/lib/util/file";
import GlyphButton from "../general/GlyphButton";
import { cn } from "@/lib/util/class";
import { usePathname } from "next/navigation";
import { extractPathFromUrl } from "@/lib/util/url";
import { InitUploadResponse } from "@/lib/types/generated/InitUploadResponse";
import { FileUpload, UploadTask } from "@/lib/types/upload";
import formatDuration from "@/lib/util/time";

const CHUNK_SIZE = 5 * 1024 * 1024;
const MAX_CONCURRENT_UPLOADS = 3;

export default function TransferWindow() {
  const [isDragOver, setIsDragOver] = useState(false);
  const [isExpanded, setIsExpanded] = useState(false);
  const [targetSize, setTargetSize] = useState(0);
  const [totalUploadedSize, setTotalUploadedSize] = useState(0);

  const path = usePathname();

  const overlayRef = useRef<HTMLDivElement>(null);

  const taskQueue = useRef<UploadTask[]>([]);
  const [fileUploads, setFileUploads] = useState<FileUpload[]>([]);

  useEffect(() => {
    const handleExternalUpload = async (e: Event) => {
      const files = (e as CustomEvent).detail as FileList;
      if (files.length > 0) await handleDrop(files);
    };

    window.addEventListener("trigger-upload", handleExternalUpload);
    return () =>
      window.removeEventListener("trigger-upload", handleExternalUpload);
  }, []);

  const handleDrop = async (e: React.DragEvent<HTMLDivElement> | FileList) => {
    const files: File[] =
      e instanceof FileList ? Array.from(e) : Array.from(e.dataTransfer.files);

    for (const file of files) {
      let size = file.size;
      let totalChunks = Math.ceil(size / CHUNK_SIZE);

      let stateUuid = crypto.randomUUID();

      let fileUpload: FileUpload = {
        stateId: stateUuid,
        total: totalChunks,
        fileId: "",
        uploadUrls: [],
        uploadId: "",
        fileName: file.name,
        bytesUploaded: 0,
        startTime: Date.now(),
        totalBytes: size,
        status: "Waiting...",
        etags: new Map<number, string>(),
      };

      setFileUploads((prev) => [...prev, fileUpload]);

      createNewUpload(file).then((uploadResponse) => {
        const fileId = uploadResponse.file_id;
        const uploadUrls = uploadResponse.upload_urls;

        setFileUploads((prev) =>
          prev.map((upload) => {
            if (upload.stateId === stateUuid) {
              return {
                ...upload,
                fileId: fileId,
                uploadUrls: uploadUrls,
                uploadId: uploadResponse.upload_id,
                status: "Uploading...",
              };
            }
            return upload;
          }),
        );

        for (let i = 1; i <= totalChunks; i++) {
          const chunk = file.slice((i - 1) * CHUNK_SIZE, i * CHUNK_SIZE);

          taskQueue.current.push({
            fileId: fileId,
            uploadUrl: uploadUrls[i - 1],
            chunkIndex: i,
            uploadId: uploadResponse.upload_id,
            chunk: chunk,
          });
        }

        setTargetSize((prev) => prev + size);
        launchWorkers();
      });
    }
  };

  const launchWorkers = async () => {
    const workerCount = Math.min(
      MAX_CONCURRENT_UPLOADS,
      taskQueue.current.length,
    );
    await Promise.all(Array.from({ length: workerCount }, runWorker));
  };

  const runWorker = async () => {
    while (taskQueue.current.length > 0) {
      const task = taskQueue.current.shift();
      if (!task) break;

      upload(task)
        .then((etag) => {
          setFileUploads((prev) => {
            const fileUpload = prev.find(
              (upload) => upload.fileId === task.fileId,
            );

            console.log("Chunk upload completed for fileId:", task.fileId, {
              etag,
              chunkIndex: task.chunkIndex,
            });

            if (!fileUpload) return prev;
            fileUpload.etags.set(task.chunkIndex, etag);

            if (fileUpload.etags.size == fileUpload.total) {
              fileUpload.status = "Completed in " + formatDuration(Date.now() - fileUpload.startTime);

              completeUpload(
                fileUpload.uploadId,
                fileUpload.fileId,
                fileUpload.etags,
              )
                .then(() => {
                  setTimeout(() => {
                    window.dispatchEvent(
                      new CustomEvent("refresh-file-list", {
                        detail: fileUpload.fileId,
                      }),
                    );
                  });
                  setTimeout(() => {
                    setFileUploads((prev) =>
                      prev.filter(
                        (upload) => upload.fileId != fileUpload.fileId,
                      ),
                    );

                    setTargetSize((prev) => prev - fileUpload.totalBytes);
                    setTotalUploadedSize(
                      (prev) => prev - fileUpload.totalBytes,
                    );
                  }, 2000);
                })
                .catch((error) => {
                  fileUpload.status = "Error";
                });
            }
            return [...prev];
          });
        })
        .catch(() => {
          const fileUpload = fileUploads.find(
            (upload) => upload.fileId === task.fileId,
          );

          if (fileUpload) {
            fileUpload.status = "Error";
          }
        });
    }
  };

  const createNewUpload = async (file: File) => {
    const uploadResponse: InitUploadResponse = await createUpload(
      file.name,
      file.size,
      file.type,
      extractPathFromUrl(path),
      CHUNK_SIZE,
    );

    return uploadResponse;
  };

  const upload = async (task: UploadTask) => {
    const uint8Array = new Uint8Array(await task.chunk.arrayBuffer());
    let uploadedBytes = 0;

    return uploadPart(
      task.uploadUrl,
      task.chunkIndex,
      uint8Array,
      (bytesSent) => {
        let newAmount = bytesSent - uploadedBytes;

        if (newAmount > 0) {
          setTotalUploadedSize((prev) => prev + newAmount);
        }

        setFileUploads((prev) =>
          prev.map((upload) => {
            if (upload.fileId === task.fileId) {
              return {
                ...upload,
                bytesUploaded: upload.bytesUploaded + newAmount,
              };
            }
            return upload;
          }),
        );

        uploadedBytes = bytesSent;
      },
    ).then((etag) => {
      return etag;
    });
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
                {fileUploads.length === 0 ? (
                  "No active transfers"
                ) : (
                  <div>
                    {fileUploads.length} upload
                    {fileUploads.length !== 1 ? "s" : ""} in progress{" "}
                    {fileUploads.length > 0
                      ? `- ${pretifyFileSize(
                          totalUploadedSize,
                        )} / ${pretifyFileSize(targetSize)}`
                      : ""}
                  </div>
                )}
              </div>
            </div>
            <div
              className={cn(styles.expandButton, isExpanded && styles.expanded)}
              onClick={() => {
                setIsExpanded(!isExpanded);
              }}
            >
              <GlyphButton glyph="chevron-up" size={24} rotate></GlyphButton>
            </div>
          </div>

          <div
            className={styles.rows}
            style={{
              maxHeight: isExpanded ? "300px" : "0px",
              minHeight: isExpanded ? "300px" : "0px",
              display: fileUploads.length === 0 ? "flex" : "block",
            }}
          >
            {fileUploads.length === 0 && (
              <div
                className={styles.subtitle}
                style={{ opacity: isExpanded ? 1 : 0 }}
              >
                <p>No active transfers</p>
              </div>
            )}
            {fileUploads
              .filter((fileProg) => fileProg.uploadId)
              .map((fileProg) => (
                <div className={styles.fileProgress} key={fileProg.uploadId}>
                  <div className={styles.progressBar}>
                    <div className={styles.progressBars}>
                      <div
                        className={styles.progressFill}
                        style={{
                          width: `${
                            fileProg.bytesUploaded > 0
                              ? (fileProg.bytesUploaded / fileProg.totalBytes) *
                                100
                              : 0
                          }%`,
                        }}
                      ></div>
                    </div>
                  </div>
                  <div className={styles.fileInfo}>
                    <div className={styles.fileName}>{fileProg.fileName}</div>
                    <p className={styles.bytesUploaded}>{fileProg.status}</p>
                  </div>
                  <div className={styles.progressNumbers}>
                    <p className={styles.bytesUploaded}>
                      {pretifyFileSize(fileProg.bytesUploaded)} /{" "}
                      {pretifyFileSize(fileProg.totalBytes)}
                    </p>
                    <p className={styles.chunksUploaded}>
                      {Math.ceil(fileProg.bytesUploaded / CHUNK_SIZE)} /{" "}
                      {fileProg.total}
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
