"use server";
import { authenticatedFetch } from "./apiClient";
import { ListFilesRequest } from "../types/generated/ListFilesRequest";
import { ListFilesResponse } from "../types/generated/ListFilesResponse";
import { ListFileElement } from "../types/generated/ListFileElement";
import { InitDownloadRequest } from "../types/generated/InitDownloadRequest";
import { InitDownloadResponse } from "../types/generated/InitDownloadResponse";
import { InitUploadRequest } from "../types/generated/InitUploadRequest";
import { InitUploadResponse } from "../types/generated/InitUploadResponse";
import { RenameFileRequest } from "../types/generated/RenameFileRequest";
import { DeleteFilesRequest } from "../types/generated/DeleteFilesRequest";
import { DeleteDirectoryRequest } from "../types/generated/DeleteDirectoryRequest";
import { CopyFilesRequest } from "../types/generated/CopyFilesRequest";
import { CopyFilesResponse } from "../types/generated/CopyFilesResponse";

export async function listFiles(directoryPath: string) {
  let request: ListFilesRequest = {
    path: directoryPath,
    limit: 1000,
    offset: 0,
  };

  const res = await authenticatedFetch(`/api/file/list/`, {
    method: "POST",
    body: JSON.stringify(request),
  });

  if (!res.ok) throw new Error("Failed to fetch file list");
  let json: ListFilesResponse = await res.json();

  let files = json.files;
  let folders: ListFileElement[] = [];
  let fileList: ListFileElement[] = [];

  folders = files.filter((file) => file.file_type === "directory");
  folders.sort((a, b) => a.file_name.localeCompare(b.file_name));

  files = files.filter((file) => file.file_type !== "directory");
  files.sort((a, b) => a.file_name.localeCompare(b.file_name));

  return { files: fileList, folders: folders };
}

export async function fetchUrl(fileId: string) {
  let request: InitDownloadRequest = {
    file_id: fileId,
  };

  const res = await authenticatedFetch(`/api/download/create`, {
    method: "POST",
    body: JSON.stringify(request),
  });

  let json: InitDownloadResponse = await res.json();

  if (!res.ok) throw new Error("Failed to fetch download URL");

  return json.download_url;
}

export async function createUpload(
  fileName: string,
  fileSize: number,
  contentType: string,
  path: string,
): Promise<InitUploadResponse> {
  let request: InitUploadRequest = {
    filename: fileName,
    size: BigInt(fileSize),
    content_type: contentType,
    path: path,
  };

  const res = await authenticatedFetch(`/api/upload/create`, {
    method: "POST",
    body: JSON.stringify(request),
  });

  let json: InitUploadResponse = await res.json();

  if (!res.ok)
    throw new Error("Failed to create upload: " + JSON.stringify(res));

  return json;
}

export async function uploadPart(
  signedUrl: string,
  partNumber: number,
  body: Uint8Array,
  checksum: string
) {
  const urlWithParams = new URL(signedUrl);
  urlWithParams.searchParams.append("partNumber", partNumber.toString());

  const response = await fetch(urlWithParams.toString(), {
    method: "PUT",
    body: body.buffer as ArrayBuffer,
    headers: {
      "Content-Type": "application/octet-stream",
      "x-amz-checksum-sha256": checksum,
    },
  });

  if (!response.ok) {
    throw new Error(
      `Upload failed for part ${partNumber}: ${response.statusText}`,
    );
  }

  return response;
};

export async function completeUpload(uploadId: string, fileId: string) {
  const response = await authenticatedFetch(`/api/upload/complete`, {
    method: "POST",
    body: JSON.stringify({
      upload_id: uploadId,
      file_id: fileId,
    }),
  });

  if (!response.ok) {
    throw new Error(`Failed to complete upload: ${response.statusText}`);
  }

  return response;
}

export async function renameFile(fileId: string, newFileName: string) {
  let request: RenameFileRequest = {
    file_id: fileId,
    file_name: newFileName,
  };

  const res = await authenticatedFetch(`/api/file/rename`, {
    method: "PATCH",
    body: JSON.stringify(request),
  });

  return res.ok;
}

export async function deleteFiles(fileIds: string[]) {
  let request: DeleteFilesRequest = {
    file_ids: fileIds,
  };

  const res = await authenticatedFetch(`/api/file/delete`, {
    method: "DELETE",
    body: JSON.stringify(request),
  });

  return res.ok;
}

export async function deleteDirectory(directoryPath: string) {
  let request: DeleteDirectoryRequest = {
    path: directoryPath,
  };

  const res = await authenticatedFetch(`/api/directory/delete`, {
    method: "DELETE",
    body: JSON.stringify(request),
  });

  return res.ok;
}

export async function copyFiles(fileIds: string[], destinationPath: string) {
  let destPath = destinationPath.startsWith("/")
    ? destinationPath.slice(1)
    : destinationPath;

  let request: CopyFilesRequest = {
    file_ids: fileIds,
    destination_path: destPath,
  };

  const res = await authenticatedFetch(`/api/file/copy`, {
    method: "POST",
    body: JSON.stringify(request),
  });

  let json: CopyFilesResponse = await res.json();

  if (!res.ok) throw new Error("Failed to copy file");

  return json.file_ids;
}
