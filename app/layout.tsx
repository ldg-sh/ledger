import styles from "./layout.module.scss";
import { Inter } from "next/font/google";

const inter = Inter({ subsets: ["latin"] });

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html className={styles.html + " " + inter.className} lang="en">
      <body className={styles.body}>{children}</body>
    </html>
  );
}
