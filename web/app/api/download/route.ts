import { ShareDownloadRequest } from "@/lib/types/generated/ShareDownloadRequest";
import { ShareDownloadResponse } from "@/lib/types/generated/ShareDownloadResponse";

const EDGE_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

export async function GET(request: Request) {
    const { searchParams } = new URL(request.url);
  const token = searchParams.get("t");

  if (!token) return new Response("Unauthorized", { status: 401 });

  const req: ShareDownloadRequest = {
    token,
  };

  const presignRes = await fetch(`${EDGE_URL}/download/share/create`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });

  if (!presignRes.ok)
    return new Response("Failed to get link: " + (await presignRes.text()), {
      status: 500,
    });

  const res: ShareDownloadResponse = await presignRes.json();

  const fileRes = await fetch(res.presigned_url);

  const contentDisposition = fileRes.headers.get("Content-Disposition");
  const fileNameMatch = contentDisposition?.match(/filename="(.+)"/);
  const fileName = fileNameMatch ? fileNameMatch[1] : "ledger-download-" + new Date().toISOString();

  return new Response(fileRes.body, {
    headers: {
      "Content-Type":
        fileRes.headers.get("Content-Type") || "application/octet-stream",
      "Content-Disposition": `attachment; filename="${fileName}"`,
      "Content-Length": fileRes.headers.get("Content-Length") || "0",
      "Cache-Control": "no-store",
    },
  });
}
