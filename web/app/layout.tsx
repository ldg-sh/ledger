import Header from "@/components/header/Header";
import styles from "./layout.module.scss";
import { Inter } from "next/font/google";
import localFont from "next/font/local";
import { cn } from "@/lib/util/class";
import { MenuProvider } from "@/context/MenuContext";
import { UserProvider } from "@/context/UserContext";
import { SortProvider } from "@/context/SortContext";
import { LoadingProvider } from "@/context/LoadingContext";

const inter = Inter({ subsets: ["latin"], variable: "--font-inter" });
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
        <UserProvider>
          <MenuProvider>
            <SortProvider>
              <LoadingProvider>
                <Header />
                {children}
              </LoadingProvider>
            </SortProvider>
          </MenuProvider>
        </UserProvider>
      </body>
    </html>
  );
}
