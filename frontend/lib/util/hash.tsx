export async function sha256_bytes(data: Uint8Array): Promise<string> {
  const hashBuffer = await crypto.subtle.digest("SHA-256", data.buffer as ArrayBuffer);

  const hashArray = Array.from(new Uint8Array(hashBuffer));

  return hashArray
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}