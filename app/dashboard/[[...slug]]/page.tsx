import Row from "@/components/browser/Row";
import styles from "./page.module.scss";
import Location from "@/components/browser/Location";
import TableHeader from "@/components/browser/TableHeader";

export default function DashboardPage() {
  return (
    <div className={styles.pageContainer}>
      <div className={styles.centerpiece}>
        <Location />
        <TableHeader />
        <div className={styles.rows}>
          <Row fileName="example.txt" fileSize={1024} fileType="text/plain" createdAt="2024-01-01T12:00:00Z" />
          <Row fileName="image.png" fileSize={123817984} fileType="image/png" createdAt="2024-02-01T12:00:00Z" />
          <Row fileName="document.pdf" fileSize={204800} fileType="application/pdf" createdAt="2024-03-01T12:00:00Z" />
          <Row fileName="archive.zip" fileSize={51200000} fileType="application/zip" createdAt="2024-04-01T12:00:00Z" />
          <Row fileName="video.mp4" fileSize={209715200} fileType="video/mp4" createdAt="2024-07-01T12:00:00Z" />
          <Row fileName="audio.mp3" fileSize={5120000} fileType="audio/mpeg" createdAt="2024-08-01T12:00:00Z" />
        </div>
      </div>
    </div>
  );
}
