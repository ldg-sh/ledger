import { FileService } from "./api/file";

export async function downloadInChunks(path: string, fileName: string, totalSize: number) {
  const CHUNK_SIZE = 5 * 1024 * 1024; 
  const totalChunks = Math.ceil(totalSize / CHUNK_SIZE);
  const chunks: Uint8Array[] = [];

  for (let i = 0; i < totalChunks; i++) {
    const start = i * CHUNK_SIZE;
    const end = Math.min(start + CHUNK_SIZE - 1, totalSize - 1);

    console.log(`Downloading chunk ${i + 1}/${totalChunks}: bytes ${start}-${end}`);

    const res = await fetch(`/api/download/${path}?range_start=${start}&range_end=${end}`);
    
    if (!res.ok) throw new Error(`Failed to download chunk ${i}`);

    const data = await res.arrayBuffer();
    chunks.push(new Uint8Array(data));
  }

  const blob = new Blob(chunks as BlobPart[], { type: "application/octet-stream" });
  const url = window.URL.createObjectURL(blob);
  
  const a = document.createElement("a");
  a.href = url;
  a.download = fileName;
  a.click();
  window.URL.revokeObjectURL(url);
}