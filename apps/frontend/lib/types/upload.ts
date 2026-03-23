interface UploadTask {
  fileId: string;
  fileName: string;
  uploadId: string;
  chunkIndex: number;
  totalChunks: number;
  chunk: Blob;
  stateKey: string;
}

interface FileProgress {
  name: string;
  percent: number;
  done: number;
  total: number;
  fileId: string;
  uploadId: string;
  fileName: string;
  bytesUploaded: number;
  totalBytes: number;
  status?: "Waiting..." | "Completed" | "Error";
  stateKey: string;
}

type ProgressMap = Record<string, FileProgress>;
