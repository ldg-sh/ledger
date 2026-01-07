"use client";

import { useRef, useState } from "react";
import styles from "./TransferWindow.module.scss";
import { createUpload, uploadPart } from "@/lib/api/file";
import { sha256_bytes } from "@/lib/util/hash";


const CHUNK_SIZE = 5 * 1024 * 1024; // 5 MB
const MAX_CONCURRENT_UPLOADS = 3;

export default function TransferWindow() {
  const [progress, setProgress] = useState<ProgressMap>({});
  const [uploading, setUploading] = useState(false);

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
        [fileId]: { name: file.name, pct: 0, done: 0, total: totalChunks },
      }));

      console.log("Created upload session:", createRes);

      for (let i = 0; i < totalChunks; i++) {
        const chunk = file.slice(i * CHUNK_SIZE, (i + 1) * CHUNK_SIZE);
        console.log(`Prepared chunk ${i + 1} of ${totalChunks} for file ${file.name}`);

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
      console.log("Launching upload workers");
      await launchWorkers();
    } else {
      console.log("Upload already in progress, workers will pick up new tasks");
    }
  };

  const launchWorkers = async () => {
    console.log("Starting upload workers");
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
          const pct = Math.floor((done / fileProg.total) * 100);
          return {
            ...prev,
            [task.fileId]: { ...fileProg, done, pct },
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

    console.log("Upload part response:", uploadRes);
  };

  return (
    <div className={styles.transferWindow} onDrop={handleDrop}>
      <input
        type="file"
        multiple
        onChange={(e) => {
          const files = e.target.files;
          console.log("File input change event:", e);
          if (files) {
            console.log("Files selected via input:", files);
            const dt = new DataTransfer();
            for (let i = 0; i < files.length; i++) {
              dt.items.add(files[i]);
            }
            const event = {
              preventDefault: () => {},
              dataTransfer: dt,
            } as unknown as React.DragEvent<HTMLDivElement>;

            handleDrop(event);
          }
        }}
      />
      Transfer Window Component
    </div>
  );
}
