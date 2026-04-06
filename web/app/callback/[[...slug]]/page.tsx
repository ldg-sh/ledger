"use client";

import { useEffect, useState } from "react";
import { useRouter, useParams, useSearchParams } from "next/navigation";
import styles from "./page.module.scss";

export default function CallbackPage() {
  const searchParams = useSearchParams();
  const params = useParams();
  const router = useRouter();
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const slug = params.slug;
    const provider = Array.isArray(slug) ? slug[0] : slug;
    const code = searchParams.get("code");

    if (!code || !provider) {
      setError("Missing authentication code or provider.");
      return;
    }

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

        document.dispatchEvent(new CustomEvent("reload-user"));
        router.push("/");
      } catch (err: any) {
        console.error("Authentication error:", err);
        setError(err.message);
      }
    };

    exchangeCode();
  }, [searchParams, params, router]);

  return (
    <div className={styles.container}>
      {error ? (
        <div className={styles.error}><strong>Failed to authenticate:</strong> {error}</div>
      ) : (
        <div className={styles.loading}>Authenticating...</div>
      )}
    </div>
  );
}
