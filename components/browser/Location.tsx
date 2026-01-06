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
      {array.length === 0 && (
        <span
          className={styles.pathSegment}
          onClick={(element) => {
            const fullPath = "/dashboard";

            router.push(fullPath);
          }}
        >
          {"/"}
        </span>
      )}
      {array.map((_, index) => (
        <span
          key={index}
          className={styles.pathSegment}
          onClick={(element) => {
            const clickedPath = array.slice(0, index + 1).join("/") || "/";
            const fullPath =
              clickedPath === "/" ? "/dashboard" : "/dashboard/" + clickedPath;

            router.push(fullPath);
          }}
        >
          {"  /  "}
          {array[index]}
        </span>
      ))}
    </div>
  );
}
