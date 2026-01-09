import { Suspense } from "react";
import Location from "@/components/browser/Location";
import TableHeader from "@/components/browser/TableHeader";
import FileList from "@/components/browser/FileList";
import styles from "./page.module.scss";
import TransferWindow from "@/components/transfer/TransferWindow";
import Footer from "@/components/browser/Footer";
import CreateFolder from "@/components/browser/popups/CreateFolder";

export default function DashboardPage() {
  return (
    <div className={styles.pageContainer}>
      <div className={styles.centerpiece}>
        <TransferWindow />
        <Location />
        <div className={styles.content}>
          <div className={styles.tableHeader}>
            <TableHeader />
          </div>

          <div className={styles.rows}>
            <Suspense
              fallback={<div className={styles.loading}>Loading files...</div>}
            >
              <FileList />
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
