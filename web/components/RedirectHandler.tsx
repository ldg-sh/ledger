"use client";
import { useEffect } from "react";
import { useRouter } from "next/navigation";
import { logout } from "@/context/UserContext";

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
