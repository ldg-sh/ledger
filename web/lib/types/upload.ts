interface UploadTask {
  fileId: string;
  fileName: string;
  uploadUrl: string;
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
  uploadUrl: string;
  uploadId: string;
  fileName: string;
  bytesUploaded: number;
  totalBytes: number;
  status?: "Waiting..." | "Uploading..." | "Completed" | "Error";
  stateKey: string;
}

type ProgressMap = Record<string, FileProgress>;