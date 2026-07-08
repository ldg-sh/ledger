"use client";

import Button from "@/components/general/Button";
import Spinner from "@/components/svg/Spinner";
import { useParams, useRouter, useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";
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
          <div className={styles.info}>
            <h1 className={styles.errorTitle}>Failed to Authenticate</h1>
            <p className={styles.errorMessage}>{error}</p>
          </div>
          <Button
            width="180px"
            label="Go Back"
            onClick={() => router.push("/login")}
          />
        </div>
      ) : (
        <Spinner height={30} />
      )}
    </div>
  );
}
