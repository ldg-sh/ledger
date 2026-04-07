"use client";

import { logout, useUser } from "@/context/UserContext";
import styles from "./User.module.scss";
import Image from "next/image";
import { AnimatePresence } from "motion/react";
import { ContextMenu } from "@/components/general/menu/ContextMenu";
import ContextMenuItem from "@/components/general/menu/ContextMenuItem";
import { useCustomMenu } from "@/hooks/customMenu";
import { useRef, useState } from "react";
import { cn } from "@/lib/util/class";

export default function User() {
  const user = useUser();
  const { visible, showMenu, hideMenu } = useCustomMenu("user-menu");
  const container = useRef<HTMLDivElement>(null);

  const [coords, setCoords] = useState({ x: 0, y: 0 });

  if (user.loading) {
    return <div className={styles.container}></div>;
  }

  if (!user.user) {
    return;
  }

  const onLoad = () => {
    if (container.current) {
      const rect = container.current.getBoundingClientRect();
      setCoords({ x: rect.left, y: rect.bottom });
    }

    window.addEventListener("resize", () => {
      if (container.current) {
        const rect = container.current.getBoundingClientRect();
        setCoords({ x: rect.left, y: rect.bottom });
      }
    });
  };

  return (
    <div
      className={styles.container}
      ref={container}
      onClick={(event) => showMenu(event)}
      onLoad={onLoad}
    >
      {user.user?.avatar_url ? (
        <Image
          src={user.user?.avatar_url}
          alt={`${user.user?.username}'s avatar`}
          className={styles.avatar}
          width={52}
          height={52}
        />
      ) : (
        null
      )}
      <button
        className={cn(styles.info, !user.user?.avatar_url && styles.noAvatar)}
        onClick={(event: React.MouseEvent<HTMLButtonElement>) =>
          showMenu(event)
        }
      >
        <h1 className={styles.title}>Logged in as</h1>
        <p className={styles.username}>{user.user?.username}</p>
      </button>
      <AnimatePresence>
        {visible && (
          <div>
            <ContextMenu
              x={coords.x + 10}
              y={coords.y + 10}
            >
              <ContextMenuItem
                label="Log Out"
                glyph="log-out"
                destructive
                onClick={() => {
                  logout();
                  hideMenu();
                }}
              />
            </ContextMenu>
          </div>
        )}
      </AnimatePresence>
    </div>
  );
}
