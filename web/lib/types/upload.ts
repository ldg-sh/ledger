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
  status: UploadStatus | null;
  etags: Map<number, string>;
}

export type UploadStatus = 
  | "Waiting..." 
  | "Finalizing..." 
  | "Uploading..." 
  | "Completed" 
  | "Error";