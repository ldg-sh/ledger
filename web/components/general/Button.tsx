"use client";

import { ReactNode, MouseEventHandler } from "react";
import styles from "./Button.module.scss";
import { cn } from "@/lib/util/class";

interface ButtonProps {
  icon: ReactNode;
  label: string;
  onClick?: MouseEventHandler<HTMLDivElement>;
  variant?: "primary" | "secondary";
  className?: string;
  href?: string;
}

export default function Button({
  icon,
  label,
  onClick,
  variant = "primary",
  className,
  href,
}: ButtonProps) {
  return (
    <div className={cn(styles.topButtonComponent,
      variant === "primary" ? styles.primary : cn(styles.secondary, styles.nonPrimaryElement),
      className,
    )}>
      {href ? <a href={href} className={styles.linkComponent}><div
        className={cn(
          styles.buttonComponent,
          variant === "primary" ? styles.primary : cn(styles.secondary, styles.nonPrimaryElement),
          className,
        )}
      >
        {icon}
        <span>{label}</span>
      </div></a> : <div
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
