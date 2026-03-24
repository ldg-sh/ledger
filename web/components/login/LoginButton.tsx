import React from "react";
import styles from "./LoginButton.module.scss";

interface LoginButtonProps {
  authUrl: string;
  title: string;
  svg: React.ReactNode;
}

export default function LoginButton({ authUrl, title, svg }: LoginButtonProps) {
  return (
    <a href={authUrl} className={styles.loginButton}>
      {svg}
      <div>{title}</div>
    </a>
  );
}
