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
import { CompleteUploadRequest } from "../types/generated/CompleteUploadRequest";

export async function listFiles(directoryPath: string) {
  let request: ListFilesRequest = {
    path: directoryPath,
    limit: 1000,
    offset: 0,
  };

  const res = await authenticatedFetch(`/file/list`, {
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

  fileList = files.filter((file) => file.file_type !== "directory");
  fileList.sort((a, b) => a.file_name.localeCompare(b.file_name));

  return { files: fileList, folders: folders };
}

export async function fetchUrl(fileId: string) {
  let request: InitDownloadRequest = {
    file_id: fileId,
  };

  const res = await authenticatedFetch(`/download/create`, {
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
  chunk_size: number,
): Promise<InitUploadResponse> {
  let request: InitUploadRequest = {
    filename: fileName,
    size: fileSize,
    content_type: contentType,
    part_count: Math.ceil(fileSize / chunk_size),
    path: path,
  };

  const res = await authenticatedFetch(`/upload/create`, {
    method: "POST",
    body: JSON.stringify(request),
  });

  let body = await res.text();

  let json: InitUploadResponse = JSON.parse(body);

  if (!res.ok)
    throw new Error("Failed to create upload: " + JSON.stringify(res));

  return json;
}

export async function uploadPart(
  signedUrl: string,
  partNumber: number,
  body: Uint8Array,
) {
  const urlWithParams = new URL(signedUrl);

  const response = await fetch(urlWithParams.toString(), {
    method: "PUT",
    body: body as BodyInit,
  });

  if (!response.ok) {
    throw new Error(
      `Upload failed for part ${partNumber}: ${response.statusText}`,
    );
  }

  const etag = response.headers.get("ETag")?.replace(/"/g, "");

  if (!etag) {
    throw new Error(`No ETag returned for part ${partNumber}`);
  }

  return etag;
}

export async function completeUpload(
  uploadId: string,
  fileId: string,
  etags: Map<number, string>,
) {
  let request: CompleteUploadRequest = {
    upload_id: uploadId,
    file_id: fileId,
    parts: Array.from(etags.entries())
      .map(([partNumber, etag]) => ({
        part_number: partNumber,
        etag: etag,
      }))
      .sort((a, b) => a.part_number - b.part_number),
  };

  const response = await authenticatedFetch(`/upload/complete`, {
    method: "POST",
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    throw new Error(`Failed to complete upload: ${response.statusText}`);
  }
}

export async function renameFile(fileId: string, newFileName: string) {
  let request: RenameFileRequest = {
    file_id: fileId,
    file_name: newFileName,
  };

  const res = await authenticatedFetch(`/file/rename`, {
    method: "PATCH",
    body: JSON.stringify(request),
  });

  return res.ok;
}

export async function deleteFiles(fileIds: string[]) {
  let request: DeleteFilesRequest = {
    file_ids: fileIds,
  };

  const res = await authenticatedFetch(`/file/delete`, {
    method: "DELETE",
    body: JSON.stringify(request),
  });

  return res.ok;
}

export async function deleteDirectory(directoryPath: string) {
  let request: DeleteDirectoryRequest = {
    path: directoryPath,
  };

  const res = await authenticatedFetch(`/directory/delete`, {
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

  const res = await authenticatedFetch(`/file/copy`, {
    method: "POST",
    body: JSON.stringify(request),
  });

  let json: CopyFilesResponse = await res.json();

  if (!res.ok) throw new Error("Failed to copy file");

  return json.file_ids;
}
