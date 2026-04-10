import { ShareDownloadRequest } from "@/lib/types/generated/ShareDownloadRequest";
import { ShareDownloadResponse } from "@/lib/types/generated/ShareDownloadResponse";

const EDGE_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const token = searchParams.get("t");
  const userAgent = request.headers.get("user-agent") || "";

  if (!token) return new Response("Unauthorized", { status: 401 });

  const req: ShareDownloadRequest = { token };
  const presignRes = await fetch(`${EDGE_URL}/download/share/create`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });

  console.log("Presign response status:", presignRes.status);

  if (!presignRes.ok) return new Response("Error", { status: 500 });
  const res: ShareDownloadResponse = await presignRes.json();

  if (userAgent.includes("Discordbot") || userAgent.includes("Twitterbot")) {
    const isVideo = res.file_type?.startsWith("video/");
    const isImage = res.file_type?.startsWith("image/");

    return new Response(
      `<html>
        <head>
          ${isImage ? `<meta property="og:image" content="${res.presigned_url}" />` : ""}
          ${
            isVideo
              ? `
            <meta property="og:type" content="video.other" />
            <meta property="og:video" content="${res.presigned_url}" />
            <meta property="og:video:type" content="${res.file_type}" />
            <meta property="og:video:width" content="1280" />
            <meta property="og:video:height" content="720" />
          `
              : ""
          }
          <meta name="twitter:card" content="${isVideo ? "player" : "summary_large_image"}" />
        </head>
        <body>Redirecting...</body>
      </html>`,
      { headers: { "Content-Type": "text/html" } },
    );
  }

  const fileRes = await fetch(res.presigned_url);

  const contentDisposition = fileRes.headers.get("Content-Disposition");
  const fileNameMatch = contentDisposition?.match(/filename="(.+)"/);
  const fileName = fileNameMatch
    ? fileNameMatch[1]
    : "ledger-download-" + new Date().toISOString();

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
