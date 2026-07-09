"use client";

import Button from "@/components/general/Button";
import Spinner from "@/components/svg/Spinner";
import { useParams, useRouter, useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";
import styles from "./page.module.scss";
import { UserRoundX } from "lucide-react";

export default function CallbackPage() {
  const searchParams = useSearchParams();
  const params = useParams();
  const router = useRouter();
  const [error, setError] = useState<string | null>(null);
  const [status, setStatus] = useState<string | null>(null);

  useEffect(() => {
    const slug = params.slug;
    const provider = Array.isArray(slug) ? slug[0] : slug;
    const code = searchParams.get("code");

    if (!code || !provider) {
      setError("Missing authentication code or provider.");
      return;
    }

    let formattedProvider =
      provider.charAt(0).toUpperCase() + provider.slice(1);
    if (provider == "github") {
      formattedProvider = "GitHub";
    }

    setStatus(`Communicating with ${formattedProvider}`);

    const exchangeCode = async () => {
      try {
        const response = await fetch(
          `/auth/callback/${provider}?code=${code}`,
          {
            method: "GET",
            credentials: "include",
          },
        );

        if (!response.ok) {
          const errorText = await response.text();

          throw new Error(errorText || "Failed to authenticate");
        }

        setStatus(`Finalizing your session`);

        document.dispatchEvent(
          new CustomEvent("reload-user", {
            detail: () => {
              router.push("/");
            },
          }),
        );
      } catch (err: unknown) {
        console.error("Authentication error:", err);
        setError((err as Error)?.message);
      }
    };

    exchangeCode();
  }, [searchParams, params, router]);

  return (
    <div className={styles.container}>
      {error ? (
        <div className={styles.error}>
          <div className={styles.loading}>
            <UserRoundX className={styles.icon} />
            <p className={styles.status}>Failed to authenticate</p>
            <p className={styles.errorMessage}>{error}</p>
          </div>
          <Button
            width="180px"
            label="Go Back"
            onClick={() => router.push("/login")}
          />
        </div>
      ) : (
        <div className={styles.loading}>
          <Spinner height={40} />
          <p className={styles.status}>{status}</p>
          <p className={styles.description}>This should only take a moment</p>
        </div>
      )}
    </div>
  );
}
