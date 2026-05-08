"use client";
import { logout } from "@/context/UserContext";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export function AuthRedirectHandler() {
  const router = useRouter();

  useEffect(() => {
    const handleAuthFailure = async () => {
      await logout();
      router.push("/login");
    };

    window.addEventListener("auth-failure", handleAuthFailure);
    return () => window.removeEventListener("auth-failure", handleAuthFailure);
  }, [router]);

  return null;
}
