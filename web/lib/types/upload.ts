export interface UploadTask {
  fileId: string;
  uploadUrl: string;
  chunkIndex: number;
  uploadId: string;
  chunk: Blob;
}

export interface FileUpload {
  stateId: string;
  total: number;
  fileId: string;
  uploadUrls: string[];
  uploadId: string;
  fileName: string;
  bytesUploaded: number;
  totalBytes: number;
  status: string;
  startTime: number;
  etags: Map<number, string>;
}