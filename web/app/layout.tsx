import Header from "@/components/header/Header";
import { AuthRedirectHandler } from "@/components/RedirectHandler";
import { LoadingProvider } from "@/context/LoadingContext";
import { MenuProvider } from "@/context/MenuContext";
import { SortProvider } from "@/context/SortContext";
import { UserProvider } from "@/context/UserContext";
import { cn } from "@/lib/util/class";
import { Metadata } from "next";
import { Inter } from "next/font/google";
import localFont from "next/font/local";
import styles from "./layout.module.scss";

const inter = Inter({ subsets: ["latin"], variable: "--font-inter" });
const overusedGrotesk = localFont({
  src: "../public/fonts/OverusedGrotesk-VF.woff",
  variable: "--font-overused-grotesk",
});

export const metadata: Metadata = {
  title: "Ledger",
  description: "File storage at the edge.",
};

export default function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html
      className={cn(
        styles.html,
        inter.className,
        inter.variable,
        overusedGrotesk.variable,
      )}
      lang="en"
    >
      <body className={styles.body}>
        <UserProvider>
          <AuthRedirectHandler />

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
