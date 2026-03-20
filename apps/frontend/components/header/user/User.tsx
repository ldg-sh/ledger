"use client";

import { useUser } from "@/context/UserContext";
import styles from "./User.module.scss";
import Image from "next/image";

export default function User() {
  let user = useUser();

  if (user.loading) {
    return (
      <div className={styles.container}>
      </div>
    );
  }

  if (!user.user) {
    return
  }

  return (
    <div className={styles.container}>
      <Image
        src={user.user?.avatar_url || "/default-avatar.png"}
        alt={`${user.user?.username}'s avatar`}
        className={styles.avatar}
        width={48}
        height={48}
      />
      <div className={styles.info}>
        <h1 className={styles.title}>Logged in as</h1>
        <p className={styles.username}>{user.user?.username}</p>
      </div>
    </div>
  );
}
