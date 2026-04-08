import { authenticatedFetch } from "../api/apiClient";

export const handleClientDownload = async (fileIds: string[], fileName?: string) => {
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

    if (res.ok && res.body) {
      const reader = res.body.getReader();
      const stream = new ReadableStream({
        start(controller) {
          function push() {
            reader.read().then(({ done, value }) => {
              if (done) {
                controller.close();
                return;
              }
              controller.enqueue(value);
              push();
            });
          }
          push();
        },
      });

      const blob = await new Response(stream).blob();
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = fileName || "ledger-archive-" + new Date().toISOString() + ".zip";
      a.click();
      window.URL.revokeObjectURL(url);
    }
  }
};