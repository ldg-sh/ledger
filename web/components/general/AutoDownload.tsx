"use client";
import { useEffect } from "react";

export default function AutoDownload({ url }: { url: string }) {
  useEffect(() => {
    const a = document.createElement("a");
    a.href = url;
    a.rel = "noopener";
    document.body.appendChild(a);
    a.click();
    a.remove();
  }, [url]);

  return null;
}
