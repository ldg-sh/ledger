import Row from "@/components/browser/Row";
import styles from "./page.module.scss";
import Header from "@/components/browser/Header";
import Location from "@/components/browser/Location";

export default function DashboardPage() {
  return (
    <div className={styles.pageContainer}>
      <div className={styles.centerpiece}>
        <h1>Dashboard</h1>

        <Location />
        <Header />
        <div className={styles.rows}>
          <Row fileName="example.txt" fileSize={1024} fileType="text/plain" createdAt="2024-01-01T12:00:00Z" />
          <Row fileName="image.png" fileSize={123817984} fileType="image/png" createdAt="2024-02-01T12:00:00Z" />
        </div>
      </div>
    </div>
  );
}
