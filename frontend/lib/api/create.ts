"use server";
import { authenticatedFetch } from "./apiClient";

export async function createFolder(path: string, folderName: string) {
  const res = await authenticatedFetch(
    `/directory/create${path}/${folderName}`,
    {
      method: "POST",
    }
  );

  return res.ok;
}
