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
  percent: number;
  done: number;
  total: number;
  fileId: string;
  uploadId: string;
  fileName: string;
}

type ProgressMap = Record<string, FileProgress>;
