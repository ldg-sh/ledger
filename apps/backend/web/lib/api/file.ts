"use server";
import { authenticatedFetch, authenticatedMultipartFetch } from "./apiClient";

export async function listFiles(directoryPath: string) {
  let request = {
    path: directoryPath,
    limit: 1000,
    offset: 0,
  };
  const res = await authenticatedFetch(`/file/list/${directoryPath}`, {
    method: "POST",
    body: JSON.stringify(request),
  });

  if (!res.ok) throw new Error("Failed to fetch file list");
  let json = await res.json();

  let files = json.files;

  let fileList: File[] = files.map((file: any) => ({
    file_id: file.file_id,
    file_name: file.file_name,
    file_size: file.file_size,
    file_type: file.file_type,
    created_at: file.created_at,
    path: file.path,
  }));

  let folders: any[];

  folders = fileList.filter(file => file.fileType === 'directory');
  folders.sort((a, b) => a.fileName.localeCompare(b.fileName));

  fileList = fileList.filter(file => file.fileType !== 'directory');
  fileList.sort((a, b) => a.fileName.localeCompare(b.fileName));

  return { files: fileList, folders: folders };
}

export async function fetchUrl(fileId: string) {
  let request = {
    file_id: fileId,
  };
  const res = await authenticatedFetch(`/download/${fileId}/view?preview=true`, {
    method: "POST",
    body: JSON.stringify(request),
  });

  if (!res.ok) throw new Error("Failed to download full file");
  const data = await res.arrayBuffer();

  return new Uint8Array(data);
}

export async function createUpload(
  fileName: string,
  fileSize: number,
  contentType: string,
  path: string
) {
  let request = {
    file_name: fileName,
    file_size: fileSize,
    content_type: contentType,
    path: path,
  }

  const res = await authenticatedFetch(
    `/upload/create`,
    {
      method: "POST",
      body: JSON.stringify(request),
    }
  );

  console.log("Create Upload Response:", res);

  if (!res.ok) throw new Error("Failed to create upload: " + JSON.stringify(res));

  return res.json();
}

export async function uploadPart(
  uploadId: string,
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
    "chunk",
    new Blob([chunkData] as BlobPart[], { type: "application/octet-stream" })
  );

  const res = await authenticatedMultipartFetch(
    `/upload/${fileId}`,
    formData
  );

  return res.ok;
}

export async function renameFile(
  fileId: string,
  newFileName: string,
) {
  let jsonData = {
    newName: newFileName,
  };

  const res = await authenticatedFetch(
    `/file/${fileId}`,
    {
      method: "PATCH",
      body: JSON.stringify(jsonData),
    }
  );

  return res.ok;
}

export async function deleteFile(
  fileId: string,
) {
  const res = await authenticatedFetch(
    `/file/${fileId}`,
    {
      method: "DELETE",
    }
  );

  return res.ok;
}

export async function deleteDirectory(
  directoryPath: string,
) {
  const res = await authenticatedFetch(
    `/directory/${directoryPath}`,
    {
      method: "DELETE",
    }
  );

  return res.ok;
}

export async function copyFile(
  fileId: string,
  destinationPath: string,
) {
  let destPath = destinationPath.startsWith("/")
    ? destinationPath.slice(1)
    : destinationPath;

  let jsonData = {
    destinationPath: destPath,
  };

  const res = await authenticatedFetch(
    `/file/${fileId}/copy`,
    {
      method: "POST",
      body: JSON.stringify(jsonData),
    }
  );

  let json = await res.json();

  if (!res.ok) throw new Error("Failed to copy file");

  return json.file_id;
}

export async function copyMultipleFiles(
  fileIds: string[],
  destinationPath: string,
) {  
  let jsonData = {
    fileIds: fileIds,
    destinationPath: destinationPath,
  };

  const res = await authenticatedFetch(
    `/bulk/copy`,
    {
      method: "POST",
      body: JSON.stringify(jsonData),
    }
  );

  let json = await res.json();

  if (!res.ok) throw new Error("Failed to copy files");

  return json.file_ids;
}