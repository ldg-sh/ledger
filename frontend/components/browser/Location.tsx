"use client";

import { usePathname } from "next/navigation";
import styles from "./Location.module.scss";
import { useRouter } from "next/navigation";
import { useEffect, useRef } from "react";

export default function Location() {
  const router = useRouter();

  const scrollRef = useRef<HTMLDivElement>(null);
  const pathname = usePathname();
  const array = pathname.split("/");

  array.shift();
  array.shift();

  useEffect(() => {
    if (scrollRef.current) {
      const el = scrollRef.current;
      el.scrollLeft = el.scrollWidth;
    }
  }, []);

  return (
    <div className={styles.locationBar} ref={scrollRef}>
      <span
        className={styles.pathSegment}
        onClick={(_) => {
          const fullPath = "/dashboard";

          router.push(fullPath);
        }}
      >
        {"home"}
      </span>
      <span className={styles.seperator}>{" / "}</span>
      {array.map((_, index) => (
        <div className={styles.pathGrouping} key={index + "-container"}>
          <span
            key={index}
            className={styles.pathSegment}
            onClick={(_) => {
              const clickedPath = array.slice(0, index + 1).join("/") || "/";
              const fullPath =
                clickedPath === "/"
                  ? "/dashboard"
                  : "/dashboard/" + clickedPath;

              router.push(fullPath);
            }}
          >
            {decodeURIComponent(array[index])}
          </span>
          <span
            key={index + "-sep"}
            className={styles.seperator}
            onClick={(_) => {
              const clickedPath = array.slice(0, index + 1).join("/") || "/";
              const fullPath =
                clickedPath === "/"
                  ? "/dashboard"
                  : "/dashboard/" + clickedPath;

              router.push(fullPath);
            }}
          >
            {"/"}
          </span>
        </div>
      ))}
    </div>
  );
}
