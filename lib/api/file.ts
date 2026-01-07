"use server";
import { authenticatedFetch, authenticatedMultipartFetch } from "./api-client";

export async function listFiles(directoryPath: string) {
  const res = await authenticatedFetch(`/list/${directoryPath}`);

  if (!res.ok) throw new Error("Failed to fetch file list");
  let json = await res.json();

  let files = json.files;
  let folders = json.folders;

  let fileList: File[] = files.map((file: any) => ({
    fileId: file.file_id,
    fileName: file.file_name,
    fileSize: file.file_size,
    fileType: file.file_type,
    createdAt: file.created_at,
    path: file.path,
  }));

  let folderList: Folder[] = folders.map((folder: any) => ({
    folderName: folder.name,
    fileCount: folder.file_count,
    size: folder.size,
  }));

  return { files: fileList, folders: folderList };
}

export async function downloadPart(
  rangeStart: number,
  rangeEnd: number,
  fileId: string
) {
  let formData = new FormData();
  formData.append("rangeStart", rangeStart.toString());
  formData.append("rangeEnd", rangeEnd.toString());

  const res = await authenticatedMultipartFetch(
    `/download/${fileId}`,
    formData
  );

  if (!res.ok) throw new Error("Failed to download file part");
  const data = await res.arrayBuffer();

  return new Uint8Array(data);
}

export async function createUpload(
  fileName: string,
  contentType: string,
  path: string
) {
  let formData = new FormData();
  formData.append("fileName", fileName);
  formData.append("contentType", contentType);

  const res = await authenticatedMultipartFetch(
    `/upload/create/${path}`,
    formData
  );

  return res.json();
}

export async function uploadPart(
  uploadId: string,
  path: string,
  fileId: string,
  checksum: string,
  chunkNumber: number,
  totalChunks: number,
  chunkData: Uint8Array
) {
  let formData = new FormData();
  formData.append("uploadId", uploadId);
  formData.append("checksum", checksum);
  formData.append("chunkNumber", chunkNumber.toString());
  formData.append("totalChunks", totalChunks.toString());
  formData.append(
    "chunkData",
    new Blob([chunkData] as BlobPart[], { type: "application/octet-stream" })
  );

  const res = await authenticatedMultipartFetch(
    `/upload/${path}/${fileId}`,
    formData
  );

  return res.ok;
}
