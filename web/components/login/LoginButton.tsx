import React from "react";
import styles from "./LoginButton.module.scss";

interface LoginButtonProps {
  authUrl?: string;
  title: string;
  svg: React.ReactNode;
  procedure?: () => void;
}

export default function LoginButton({ authUrl, title, svg, procedure }: LoginButtonProps) {
  return (
    <a href={authUrl} onClick={procedure} className={styles.loginButton}>
      {svg}
      <div>{title}</div>
    </a>
  );
}
