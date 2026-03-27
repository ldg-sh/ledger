"use client";
import { Suspense, useRef } from "react";
import Location from "@/components/browser/Location";
import TableHeader from "@/components/browser/TableHeader";
import FileList from "@/components/browser/FileList";
import styles from "./page.module.scss";
import TransferWindow from "@/components/transfer/TransferWindow";
import Footer from "@/components/browser/Footer";

export default function MainPage() {
  const parentContainerRef = useRef<HTMLDivElement>(null);
  return (
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
              fallback={<div className={styles.loading}>Loading files...</div>}
            >
              <FileList parentContainerRef={parentContainerRef as React.RefObject<HTMLDivElement>} />
            </Suspense>
          </div>
          <div className={styles.footer}>
            <Footer />
          </div>
        </div>
      </div>
    </div>
  );
}
