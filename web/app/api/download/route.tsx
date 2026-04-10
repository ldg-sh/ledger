import { ImageResponse } from "next/og";
import { ShareDownloadRequest } from "@/lib/types/generated/ShareDownloadRequest";
import { ShareDownloadResponse } from "@/lib/types/generated/ShareDownloadResponse";
import { join } from "path";
import { readFile } from "fs/promises";
import { pretifyFileSize } from "@/lib/util/file";

const EDGE_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const token = searchParams.get("t");
  const accept = request.headers.get("accept") || "";
  const userAgent = request.headers.get("user-agent") || "";

  if (!token) return new Response("Unauthorized", { status: 401 });

  const req: ShareDownloadRequest = { token };
  const presignRes = await fetch(`${EDGE_URL}/download/share/create`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });

  console.log("Presign response status:", presignRes.status);

  if (!presignRes.ok)
    return new Response("Error fetching metadata", { status: 500 });
  const res: ShareDownloadResponse = await presignRes.json();

  const formattedDate = res.created_at
    ? new Date(res.created_at).toLocaleString(undefined, {
        year: "numeric",
        month: "long",
        day: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      })
    : "";

  if (userAgent.includes("Discordbot") || userAgent.includes("Twitterbot")) {
    const isVideo = res.file_type?.startsWith("video/");
    const isImage = res.file_type?.startsWith("image/");

    if (accept.includes("image/")) {
      return new ImageResponse(
        <div
          style={{
            height: "100%",
            width: "100%",
            display: "flex",
            backgroundColor: "white",
            position: "relative",
            fontFamily: "'Overused Grotesk', sans-serif",
          }}
        >
          <svg
            width="1200"
            height="600"
            viewBox="0 0 1200 600"
            fill="none"
            style={{ position: "absolute" }}
          >
            <rect width="1200" height="600" fill="white" />
            <path
              d="M1004 98.4185H1023.47V144.527H1067.53V161.946H1087V225.473H961.995V289H896.419V269.532H879V206.005H942.527V223.424H959.946V206.005H942.527V81H1004V98.4185ZM942.527 269.532H898.468V286.951H959.946V225.473H942.527V269.532ZM1067.53 206.005H961.995V223.424H1084.95V163.995H1067.53V206.005ZM1004 144.527H1021.42V100.468H1004V144.527Z"
              fill="#F0F0F0"
            />
          </svg>

          <div
            style={{
              display: "flex",
              flexDirection: "column",
              padding: "84px",
            }}
          >
            <span
              style={{
                fontSize: 119,
                marginTop: -40,
                marginBottom: -10,
                fontWeight: 900,
                color: "#2A2A2A",
                fontFamily: "Overused Grotesk",
              }}
            >
              Ledger
            </span>
            <span
              style={{
                fontSize: 21,
                fontWeight: "bold",
                color: "#2A2A2A",
                marginTop: 10,
                fontFamily: "Inter",
              }}
            >
              Fast and efficient file storage.
            </span>
            <span
              style={{
                fontSize: 21,
                color: "#858585",
                maxWidth: "600px",
                fontFamily: "Inter",
              }}
            >
              Web storage at the edge, providing you with the fastest possible
              speeds in a simplistic interface.
            </span>
          </div>

          <div
            style={{
              display: "flex",
              position: "absolute",
              left: 84,
              top: 380,
            }}
          >
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                width: "360px",
              }}
            >
              <span
                style={{ fontSize: 16, fontWeight: 800, fontFamily: "Inter" }}
              >
                FILE NAME
              </span>
              <span
                style={{
                  fontSize: 21,
                  color: "#858585",
                  fontFamily: "Inter",
                  overflow: "hidden",
                  textOverflow: "ellipsis",
                  whiteSpace: "nowrap",
                  width: "320px",
                }}
              >
                {res.file_name}
              </span>
            </div>
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                width: "360px",
              }}
            >
              <span
                style={{ fontSize: 16, fontWeight: 800, fontFamily: "Inter" }}
              >
                FILE SIZE
              </span>
              <span
                style={{
                  fontSize: 21,
                  color: "#858585",
                  fontFamily: "Inter",
                }}
              >
                {pretifyFileSize(res.file_size) || "Unknown"}
              </span>
            </div>
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                width: "360px",
              }}
            >
              <span
                style={{ fontSize: 16, fontWeight: 800, fontFamily: "Inter" }}
              >
                CREATED AT
              </span>
              <span
                style={{
                  fontSize: 21,
                  color: "#858585",
                  fontFamily: "Inter",
                }}
              >
                {formattedDate || "Unknown"}
              </span>
            </div>
          </div>

          {res.file_type && (
            <div
              style={{
                display: "flex",
                position: "absolute",
                left: 84,
                top: 460,
              }}
            >
              <div
                style={{
                  display: "flex",
                  flexDirection: "column",
                  width: "360px",
                }}
              >
                <span
                  style={{
                    fontSize: 16,
                    fontWeight: 800,
                    fontFamily: "Inter",
                  }}
                >
                  FILE TYPE
                </span>
                <span
                  style={{
                    fontSize: 21,
                    color: "#858585",
                    fontFamily: "Inter",
                  }}
                >
                  {res.file_type}
                </span>
              </div>
            </div>
          )}
        </div>,
        {
          width: 1200,
          height: 600,
          fonts: [
            {
              name: "Overused Grotesk",
              data: await readFile(
                join(process.cwd(), "public/fonts/OverusedGrotesk-Black.ttf"),
              ),
              weight: 900,
              style: "normal",
            },
            {
              name: "Inter",
              data: await readFile(
                join(process.cwd(), "public/fonts/Inter-Regular.ttf"),
              ),
              weight: 400,
              style: "normal",
            },
            {
              name: "Inter",
              data: await readFile(
                join(process.cwd(), "public/fonts/Inter-Extrabold.ttf"),
              ),
              weight: 800,
              style: "normal",
            },
          ],
        },
      );
    }

    if (isVideo || isImage) {
      return new Response(
        `<html>
        <head>
          <meta property="og:site_name" content=" " /> 
          ${isImage ? `<meta property="og:image" content="${res.presigned_url}" />` : ""}
          ${
            isVideo
              ? `
            <meta property="og:type" content="video.other" />
            <meta property="og:video" content="${res.presigned_url}" />
            <meta property="og:video:type" content="${res.file_type}" />
            <meta property="og:video:width" content="auto" />
            <meta property="og:video:height" content="auto" />
          `
              : ""
          }
        </head>
      </html>`,
        { headers: { "Content-Type": "text/html" } },
      );
    } else {
      return new Response(
        `<!DOCTYPE html>
      <html>
        <head>
          <meta charset="UTF-8">
          <title>${res.file_name}</title>
          <meta property="og:title" content="${res.file_name}" />
          <meta property="og:description" content="Type: ${res.file_type} • Size: ${pretifyFileSize(res.file_size)}" />
          <meta property="og:site_name" content="Ledger" />
          <meta property="og:image" content="${request.url}" />
          <meta property="og:image:width" content="1200" />
          <meta property="og:image:height" content="600" />
          <meta name="twitter:card" content="summary_large_image" />
          <meta name="theme-color" content="#2A2A2A" />
        </head>
      </html>`,
        { headers: { "Content-Type": "text/html" } },
      );
    }
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
