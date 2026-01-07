interface UploadTask {
  fileId: string;
  fileName: string;
  uploadId: string;
  chunkIndex: number;
  totalChunks: number;
  chunk: Blob;
}

interface FileProgress {
  name: string;
  pct: number;
  done: number;
  total: number;
}

type ProgressMap = Record<string, FileProgress>;
