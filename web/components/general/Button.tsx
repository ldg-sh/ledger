"use client";

import { ReactNode, MouseEventHandler } from "react";
import styles from "./Button.module.scss";
import { cn } from "@/lib/util/class";
import Link from "next/link";

interface ButtonProps {
  icon?: ReactNode;
  label: string;
  onClick?: MouseEventHandler<HTMLDivElement>;
  variant?: "primary" | "secondary";
  className?: string;
  href?: string;
  width?: string;
  height?: string;
}

export default function Button({
  icon,
  label,
  onClick,
  variant = "primary",
  className,
  href,
  width,
  height,
}: ButtonProps) {
  return (
    <div className={cn(styles.topButtonComponent,
      variant === "primary" ? styles.primary : cn(styles.secondary, styles.nonPrimaryElement),
      className,
    )} style={{ width, height }}>
      {href ? <Link href={href} className={styles.linkComponent}><div
        className={cn(
          styles.buttonComponent,
          variant === "primary" ? styles.primary : cn(styles.secondary, styles.nonPrimaryElement),
          className,
        )}
      >
        {icon}
        <span>{label}</span>
      </div></Link> : <div
        className={cn(
          styles.buttonComponent,
          className,
        )}
        onClick={onClick}
      >
        {icon}
        <span>{label}</span>
      </div>}
    </div>
  );
}
