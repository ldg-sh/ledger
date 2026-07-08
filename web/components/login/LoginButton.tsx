import { cn } from "@/lib/util/class";
import React from "react";
import styles from "./LoginButton.module.scss";
import Image from "next/image";

interface LoginButtonProps {
  authUrl?: string;
  title?: string;
  svg?: React.ReactNode;
  image?: string;
  procedure?: () => void;
  isLoading?: boolean;
  bold?: boolean;
}

export default function LoginButton({
  authUrl,
  title,
  svg,
  image,
  procedure,
  isLoading = false,
  bold = false,
}: LoginButtonProps) {
  const isLink = !!authUrl;
  const Tag = isLink ? "a" : "button";

  return (
    <Tag
      href={authUrl}
      type={isLink ? undefined : "submit"}
      disabled={isLoading}
      onClick={(e) => {
        if (procedure) {
          e.preventDefault();
          procedure();
        }
      }}
      className={cn(
        styles.loginButton,
        isLoading && styles.loading,
        bold && styles.boldButton,
        isLink && styles.linkButton,
      )}
      aria-busy={isLoading}
    >
      {React.isValidElement(svg)
        ? React.cloneElement(
            svg as React.ReactElement,
            { "aria-hidden": "true" } as React.HTMLAttributes<HTMLElement>,
          )
        : svg}
      {image ? <Image src={image} width={50} height={50} alt={title ?? ""} className={styles.image} /> : null}
      {title ? <span className={styles.span}>{title}</span> : null}
    </Tag>
  );
}
