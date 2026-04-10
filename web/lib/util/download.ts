"use client";

import { authenticatedFetch } from "../api/apiClient";

export const handleClientDownload = async (
  fileIds: string[],
  fileName?: string,
) => {
  if (typeof window === "undefined") return;

  const streamSaver = (await import("streamsaver")).default;
  streamSaver.mitm = window.location.origin + "/mitm.html";

  if (!document) {
    console.error("Document is not defined. Cannot initiate download.");
  }

  if (fileIds.length === 0) return;

  if (fileIds.length === 1 && fileName) {
    const res = await authenticatedFetch(`/download/create`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ file_id: fileIds[0], file_name: fileName }),
    });

    if (res.ok) {
      const { download_url } = await res.json();
      window.location.assign(download_url);
    }
  } else {
    const res = await authenticatedFetch(`/file/zip`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ item_ids: fileIds }),
    });

    if (!res.ok || !res.body) {
      console.error("Failed to start download stream");
      return;
    }

    const totalSize = Number(res.headers.get("x-archive-size")) || 0;

    const fileStream = streamSaver.createWriteStream(
      fileName || "ledger-archive-" + new Date().toISOString() + ".zip",
      {
        size: totalSize,
        writableStrategy: undefined,
        readableStrategy: undefined,
      },
    );

    if (res.body.pipeTo) {
      return res.body.pipeTo(fileStream);
    }

    const writer = fileStream.getWriter();
    const reader = res.body.getReader();
    const pump: () => Promise<void> = () =>
      reader
        .read()
        .then((res) =>
          res.done ? writer.close() : writer.write(res.value).then(pump),
        );

    return pump();
  }
};
