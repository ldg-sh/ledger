"use client";

import { usePathname } from "next/navigation";
import styles from "./location.module.scss";
import { useRouter } from "next/navigation";

export default function Location() {
  const router = useRouter();

  const pathname = usePathname();
  const array = pathname.split("/");

  array.shift();
  array.shift();

  return (
    <div className={styles.locationBar}>
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
        <div key={index + "-container"}>
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
            {array[index]}
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
