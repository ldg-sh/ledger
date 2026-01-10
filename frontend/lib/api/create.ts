"use server";
import { authenticatedFetch } from "./api-client";

export async function createFolder(path: string, folderName: string) {
  console.log(`/create/directory${path}/${folderName}`);
  const res = await authenticatedFetch(
    `/create/directory${path}/${folderName}`,
    {
      method: "POST",
    }
  );


  return res.ok;
}
