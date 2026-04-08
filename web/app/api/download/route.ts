"use client";

import { authenticatedFetch } from "@/lib/api/apiClient";

import { InitDownloadRequest } from "@/lib/types/generated/InitDownloadRequest";

import { ZipRequest } from "@/lib/types/generated/ZipRequest";

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const preview = searchParams.get("preview") === "true";
  const idsParam = searchParams.get("ids");
  const fileIds = idsParam ? idsParam.split(",") : [];

  if (fileIds.length === 0) {
    return new Response("No file specified", { status: 400 });
  }

  let res: Response;

  if (fileIds.length === 1) {
    const fileName = searchParams.get("name") || "file";

    const req: InitDownloadRequest = {
      file_id: fileIds[0],
      file_name: fileName,
    };

    const presignRes = await authenticatedFetch(`/download/create`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(req),
    });

    if (!presignRes.ok) {
      return new Response("Error", { status: presignRes.status });
    }

    const { download_url } = await presignRes.json();

    res = await fetch(download_url);
  } else {
    const req: ZipRequest = {
      item_ids: fileIds,
    };

    res = await authenticatedFetch(`/file/zip`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(req),
    });
  }

  if (!res.ok) return new Response("Error", { status: res.status });

  const headers = new Headers();

  headers.set(
    "Content-Type",
    res.headers.get("Content-Type") || "application/octet-stream",
  );

  const totalSize = res.headers.get("Content-Length");

  if (totalSize) {
    headers.set("Content-Length", totalSize);
  }

  const fileName =
    res.headers
      .get("Content-Disposition")
      ?.split("filename=")[1]
      ?.replace(/"/g, "") || "file";

  headers.set(
    "Content-Disposition",
    preview ? "inline" : `attachment; filename="${fileName}"`,
  );

  headers.set("X-Content-Type-Options", "nosniff");
  headers.set("X-Accel-Buffering", "no");
  headers.set("Cache-Control", "no-transform");

  return new Response(res.body, { headers });
}
