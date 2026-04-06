"use client";

import { Suspense, useRef, use } from "react";
import Location from "@/components/browser/Location";
import TableHeader from "@/components/browser/TableHeader";
import FileList from "@/components/browser/FileList";
import styles from "./page.module.scss";
import TransferWindow from "@/components/transfer/TransferWindow";
import Footer from "@/components/browser/Footer";
import { FileProvider } from "@/context/FileExplorerContext";

export default function MainPage({ 
  searchParams 
}: { 
  searchParams: Promise<{ folder?: string }> 
}) {
  const resolvedParams = use(searchParams);
  const folder = resolvedParams.folder || "";

  const parentContainerRef = useRef<HTMLDivElement>(null);

  return (
    <FileProvider initialPath={folder}>
      <div className={styles.pageContainer}>
        <div className={styles.centerpiece}>
          <TransferWindow />
          <Location />
          <div className={styles.content}>
            <div className={styles.tableHeader}>
              <TableHeader />
            </div>

            <div className={styles.rows} ref={parentContainerRef}>
              <Suspense
                fallback={
                  <div className={styles.loading}>Loading files...</div>
                }
              >
                <FileList parentContainerRef={parentContainerRef} />
              </Suspense>
            </div>
            <div className={styles.footer}>
              <Footer />
            </div>
          </div>
        </div>
      </div>
    </FileProvider>
  );
}