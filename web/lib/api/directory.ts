"use client";
import { DeleteDirectoryRequest } from "../types/generated/DeleteDirectoryRequest";
import { DirectoryRequest } from "../types/generated/DirectoryRequest";
import { authenticatedFetch } from "./apiClient";

export async function createDirectory(path: string, folderName: string) {
  let directoryRequest: DirectoryRequest = {
    path: path,
    name: folderName,
  };

  const res = await authenticatedFetch(`/directory/create`, {
    method: "POST",
    body: JSON.stringify(directoryRequest),
  });

  return res.ok;
}

export async function deleteDirectory(directoryPath: string) {
  let directoryDeleteRequest: DeleteDirectoryRequest = {
    path: directoryPath,
  };

  const res = await authenticatedFetch(`/directory/delete`, {
    method: "DELETE",
    body: JSON.stringify(directoryDeleteRequest),
  });

  return res.ok;
}

export async function copyDirectory(
  directoryPath: string,
  destinationPath: string,
) {
  let destPath = destinationPath.startsWith("/")
    ? destinationPath.slice(1)
    : destinationPath;

  let jsonData = {
    destinationPath: destPath,
  };

  const res = await authenticatedFetch(`/directory/copy/${directoryPath}`, {
    method: "POST",
    body: JSON.stringify(jsonData),
  });

  let json = await res.json();

  return json.directory_id;
}

export async function moveDirectory(
  directoryPath: string,
  destinationPath: string,
) {
  let destPath = destinationPath.startsWith("/")
    ? destinationPath.slice(1)
    : destinationPath;

  let jsonData = {
    destinationPath: destPath,
  };

  const res = await authenticatedFetch(`/directory/move/${directoryPath}`, {
    method: "POST",
    body: JSON.stringify(jsonData),
  });

  let json = await res.json();
  return json.success;
}

export async function renameDirectory(directoryPath: string, newName: string) {
  let jsonData = {
    newName: newName,
  };

  const res = await authenticatedFetch(`/directory/rename/${directoryPath}`, {
    method: "POST",
    body: JSON.stringify(jsonData),
  });

  let json = await res.json();
  return json.success;
}
