import { ShareDownloadRequest } from "@/lib/types/generated/ShareDownloadRequest";
import { ShareDownloadResponse } from "@/lib/types/generated/ShareDownloadResponse";
import { pretifyFileSize } from "@/lib/util/file";
import type { Metadata } from "next";
import styles from "./page.module.scss";
import Button from "@/components/general/Button";
import { Download } from "lucide-react";
import AutoDownload from "@/components/general/AutoDownload";


const EDGE_URL = process.env.NEXT_PUBLIC_EDGE_URL || "http://localhost:8787";

interface SharePageProps {
  searchParams: Promise<{ t?: string }>;
}

async function getShare(token: string): Promise<ShareDownloadResponse | null> {
  const req: ShareDownloadRequest = { token };

  const res = await fetch(`${EDGE_URL}/download/share/create`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(req),
    cache: "no-store",
  });

  if (!res.ok) return null;
  return res.json();
}

function formatDate(createdAt?: string) {
  if (!createdAt) return "Unknown";
  return new Date(createdAt).toLocaleString(undefined, {
    year: "numeric",
    month: "long",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export async function generateMetadata({
  searchParams,
}: SharePageProps): Promise<Metadata> {
  const { t } = await searchParams;
  if (!t) return { title: "Ledger" };

  const share = await getShare(t);
  if (!share) return { title: "Ledger" };

  const ogImageUrl = `/share/opengraph-image?t=${encodeURIComponent(t)}`;
  const isVideo = share.file_type?.startsWith("video/");

  return {
    metadataBase: new URL("https://ldg.sh"),
    title: share.file_name ? `${share.file_name} ● Ledger` : "Ledger",
    openGraph: {
      title: share.file_name || "Ledger",
      siteName: "Ledger",
      type: isVideo ? "video.other" : "website",
      images: [{ url: ogImageUrl, width: 1200, height: 600 }],
      ...(isVideo && {
        videos: [{ url: share.presigned_url, type: share.file_type }],
      }),
    },
    twitter: { card: "summary_large_image" },
  };
}

export default async function SharePage({ searchParams }: SharePageProps) {
  const { t } = await searchParams;
  if (!t) return null;

  const share = await getShare(t);
  if (!share) return null;

  const downloadUrl = `/api/share/download?t=${encodeURIComponent(t)}`;

  const rows: Array<[string, string]> = [
    ["File Name", share.file_name || "Unknown"],
    ["File Size", pretifyFileSize(share.file_size) || "Unknown"],
    ["File Type", share.file_type || "Unknown"],
    ["Created", formatDate(share.created_at)],
    ["Owner", share.owner || "Unknown"],
  ];

  return (
    <main className={styles.page}>
      <AutoDownload url={downloadUrl} />
      <div className={styles.content}>
        <div className={styles.header}>
          <h1 className={styles.title}>Your download will begin shortly...</h1>
          <p className={styles.subtitle}>
            Uploaded by <strong>{share.owner || "Unknown"}</strong> on <strong>{formatDate(share.created_at).split(" at ")[0]}</strong> at <strong>{formatDate(share.created_at).split(" at ")[1]}</strong>.
          </p>
        </div>

        <dl className={styles.table}>
          {rows.map(([label, value]) => (
            <div className={styles.row} key={label}>
              <dt className={styles.label}>{label}</dt>
              <dd className={styles.value} title={value}>
                {value}
              </dd>
            </div>
          ))}
        </dl>

        <div className={styles.perforation} aria-hidden="true" />

        <div className={styles.buttonContainer}>
          <Button
            icon={<Download size={14} strokeWidth={2.5} />}
          label="Retry Download"
          variant="primary"
          href={downloadUrl}
          />
        </div>
      </div>
      <div className={styles.footer}>
        <div className={styles.innerFooter}>
          <p className={styles.footerText}>
            © {new Date().getFullYear()} Ledger. All rights reserved. Built by{" "}
            <a
              href="https://thesamgordon.com"
              target="_blank"
              rel="noopener noreferrer"
              className={styles.footerLink}
            >
              Sam Gordon
            </a>
            .
          </p>
        </div>
      </div>
    </main>
  );
}
