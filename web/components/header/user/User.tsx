"use client";

import { logout, useUser } from "@/context/UserContext";
import styles from "./User.module.scss";
import Image from "next/image";
import { AnimatePresence } from "motion/react";
import { ContextMenu } from "@/components/general/menu/ContextMenu";
import ContextMenuItem from "@/components/general/menu/ContextMenuItem";
import { useCustomMenu } from "@/hooks/customMenu";
import { useEffect, useRef, useState } from "react";
import { cn } from "@/lib/util/class";
import { useRouter } from "next/navigation";

export default function User() {
  const user = useUser();
  const router = useRouter();
  const { visible, showMenu, hideMenu } = useCustomMenu("user-menu");
  const container = useRef<HTMLDivElement>(null);
  const [isLoadingLogout, setIsLoadingLogout] = useState(false);

  const [coords, setCoords] = useState({ x: 0, y: 0 });

  useEffect(() => {
    const updateCoords = () => {
      if (container.current) {
        const rect = container.current.getBoundingClientRect();
        setCoords({ x: rect.left, y: rect.bottom });
      } else {
        console.warn("User menu container not found for positioning.");
      }
    };

    updateCoords();

    window.addEventListener("resize", updateCoords);
    window.addEventListener("scroll", updateCoords, true);

    return () => {
      window.removeEventListener("resize", updateCoords);
      window.removeEventListener("scroll", updateCoords, true);
    };
  }, [user.loading]);

  if (!user.user) {
    return;
  }

  return (
    <div
      className={styles.container}
      ref={container}
      onClick={(event) => showMenu(event)}
    >
      {user.user?.avatar_url ? (
        <Image
          src={user.user?.avatar_url}
          alt={`${user.user?.username}'s avatar`}
          className={styles.avatar}
          width={52}
          height={52}
        />
      ) : null}
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
            <ContextMenu x={coords.x + 10} y={coords.y + 10}>
              <ContextMenuItem
                label="Log Out"
                glyph="log-out"
                destructive
                isLoading={isLoadingLogout}
                onClick={async () => {
                  setIsLoadingLogout(true);
                  await logout();
                  router.push("/login");
                  setIsLoadingLogout(false);

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
