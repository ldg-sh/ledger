import { ShareDownloadRequest } from "@/lib/types/generated/ShareDownloadRequest";
import { ShareDownloadResponse } from "@/lib/types/generated/ShareDownloadResponse";
import { pretifyFileSize } from "@/lib/util/file";
import { readFile } from "fs/promises";
import { ImageResponse } from "next/og";
import { join } from "path";

const EDGE_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const token = searchParams.get("t");
  if (!token) return new Response("Unauthorized", { status: 401 });

  const req: ShareDownloadRequest = { token };
  const presignRes = await fetch(`${EDGE_URL}/download/share/create`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
  });

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
    : "Unknown";

  const rows = [
    { label: "File Name", value: res.file_name || "Unknown" },
    { label: "File Size", value: pretifyFileSize(res.file_size) || "Unknown" },
    { label: "File Type", value: res.file_type || "Unknown" },
    { label: "Uploaded", value: formattedDate },
    { label: "Owner", value: res.owner || "Unknown" },
  ];

  return new ImageResponse(
    <div
      style={{
        height: "100%",
        width: "100%",
        display: "flex",
        flexDirection: "column",
        justifyContent: "center",
        alignItems: "center",
        backgroundColor: "#121212",
        fontFamily: "'Inter', sans-serif",
      }}
    >
      <div
        style={{
          width: "80%",
          height: "100%",
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            width: "100%",
            maxWidth: 1200,
            gap: 16,
            alignContent: "center",
            justifyContent: "space-between",
            marginTop: "-30px",
          }}
        >
          <div
            style={{
              display: "flex",
              flexDirection: "column",
            }}
          >
            <span
              style={{
                fontFamily: "'Overused Grotesk'",
                fontWeight: 800,
                fontSize: 38,
                color: "#ffffff",
                margin: 0,
                letterSpacing: "-0.03em"
              }}
            >
              Shared Download
            </span>
            <span
              style={{ fontFamily: "Inter", fontSize: 17, color: "#e0e0e0" }}
            >
              Download a file shared on Ledger.
            </span>
          </div>
          <svg width="74" viewBox="130 85 90 75" fill="none">
            <path
              d="M180.192 98.3307H185.777V111.557H198.415V116.554H204V134.777H168.142V153H149.331V147.416H144.334V129.192H162.557V134.189H167.554V129.192H162.557V93.3341H180.192V98.3307ZM162.557 147.416H149.918V152.412H167.554V134.777H162.557V147.416ZM198.415 129.192H168.142V134.189H203.412V117.142H198.415V129.192ZM180.192 111.557H185.189V98.9186H180.192V111.557Z"
              fill="white"
            />
          </svg>
        </div>

        <div
          style={{
            display: "flex",
            flexDirection: "column",
            marginTop: 40,
            backgroundColor: "#161616",
            borderRadius: 11,
            border: "1px solid #333333",
            overflow: "hidden",
            width: "100%",
          }}
        >
          {rows.map((row, i) => (
            <div
              key={row.label}
              style={{
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
                height: 59,
                padding: "0 21px",
                borderBottom:
                  i < rows.length - 1 ? "1px solid #333333" : "none",
              }}
            >
              <span
                style={{
                  fontFamily: "Inter",
                  fontSize: 17,
                  fontWeight: 400,
                  color: "#858585",
                }}
              >
                {row.label}
              </span>
              <span
                style={{
                  fontFamily: "Inter",
                  fontSize: 17,
                  fontWeight: 500,
                  color: "#e0e0e0",
                  maxWidth: 400,
                  overflow: "hidden",
                  textOverflow: "ellipsis",
                  whiteSpace: "nowrap",
                }}
              >
                {row.value}
              </span>
            </div>
          ))}
        </div>
      </div>
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
          weight: 800,
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
            join(process.cwd(), "public/fonts/Inter-Medium.ttf"),
          ),
          weight: 500,
          style: "normal",
        },
        {
          name: "Inter",
          data: await readFile(
            join(process.cwd(), "public/fonts/Inter-SemiBold.ttf"),
          ),
          weight: 600,
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
