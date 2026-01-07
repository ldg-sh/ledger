import { Suspense } from "react";
import Location from "@/components/browser/Location";
import TableHeader from "@/components/browser/TableHeader";
import FileList from "@/components/browser/FileList";
import styles from "./page.module.scss";

export default function DashboardPage() {
  return (
    <div className={styles.pageContainer}>
      <div className={styles.centerpiece}>
        <Location />
        <div className={styles.content}>
          <TableHeader />
          
          <div className={styles.rows}>
            <Suspense fallback={<div className={styles.loading}>Loading files...</div>}>
              <FileList />
            </Suspense>
          </div>
        </div>
      </div>
    </div>
  );
}