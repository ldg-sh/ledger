import Logo from "@/components/header/Header";
import styles from "./layout.module.scss";
import { Inter } from "next/font/google";
import localFont from "next/font/local";
import { cn } from "@/lib/util/class";

const inter = Inter({ subsets: ["latin"] });
const overusedGrotesk = localFont({
  src: "../public/fonts/OverusedGrotesk-VF.woff",
  variable: "--font-overused-grotesk",
});

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html
      className={cn(styles.html, inter.className, overusedGrotesk.variable)}
      lang="en"
    >
      <body className={styles.body}>
        <Logo />
        {children}
      </body>
    </html>
  );
}
