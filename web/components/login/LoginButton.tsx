import React, { useState } from "react";
import styles from "./LoginButton.module.scss";
import { cn } from "@/lib/util/class";

interface LoginButtonProps {
  authUrl?: string;
  title?: string;
  svg: React.ReactNode;
  procedure?: () => void;
  isLoading?: boolean;
  bold?: boolean;
}

export default function LoginButton({
  authUrl,
  title,
  svg,
  procedure,
  isLoading = false,
  bold = false,
}: LoginButtonProps) {
  return (
    <a
      href={authUrl}
      onClick={() => {
        if (procedure) {
          procedure();
        }
      }}
      className={cn(styles.loginButton, isLoading && styles.loading, bold && styles.boldButton)}
    >
      {svg}
      {title ? <div>{title}</div> : null}
    </a>
  );
}
