import { authenticatedFetch, authenticatedMultipartFetch } from "./api-client";

export const FileService = {
  list: async () => {
    const res = await authenticatedFetch("/list/");
    return res.json();
  },

  downloadPart: async (rangeStart: number, rangeEnd: number, fileId: string) => {
    let formData = new FormData();
    formData.append("rangeStart", rangeStart.toString());
    formData.append("rangeEnd", rangeEnd.toString());

    const res = await authenticatedMultipartFetch(`/download/${fileId}`, formData);

    if (!res.ok) throw new Error("Failed to download file part");
    const data = await res.arrayBuffer();

    return new Uint8Array(data);
  },

  createUpload: async (fileName: string, contentType: string, path: string) => {
    let formData = new FormData();
    formData.append("fileName", fileName);
    formData.append("contentType", contentType);

    const res = await authenticatedMultipartFetch(`/upload/create/${path}`, formData);

    return res.json();
  },

  uploadPart: async (uploadId: string, path: string, fileId: string, checksum: string, chunkNumber: number, totalChunks: number, chunkData: Uint8Array) => {
    let formData = new FormData();
    formData.append("uploadId", uploadId);
    formData.append("checksum", checksum);
    formData.append("chunkNumber", chunkNumber.toString());
    formData.append("totalChunks", totalChunks.toString());
    formData.append("chunkData", new Blob([chunkData] as BlobPart[], { type: "application/octet-stream" }));

    const res = await authenticatedMultipartFetch(`/upload/${path}/${fileId}`, formData);

    return res.ok;
  }
};