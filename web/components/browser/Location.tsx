"use client";

import { usePathname } from "next/navigation";
import styles from "./Location.module.scss";
import { useRouter } from "next/navigation";
import { useEffect, useRef } from "react";
import Image from "next/image";
import Spinner from "../svg/Spinner";
import { useLoading } from "@/context/LoadingContext";

export default function Location() {
  const router = useRouter();
  const { loading } = useLoading();

  const scrollRef = useRef<HTMLDivElement>(null);
  const pathname = usePathname();
  const array = pathname.split("/");
  array.shift();

  if (array.length == 1 && array[0] === "") {
    array.pop();
  }

  useEffect(() => {
    if (scrollRef.current) {
      const el = scrollRef.current;
      el.scrollLeft = el.scrollWidth;
    }
  }, []);

  return (
    <div className={styles.locationBar} ref={scrollRef}>
      <div className={styles.left}>
        <span
          className={styles.pathSegment}
          onClick={(_) => {
            const fullPath = "/";

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
                const fullPath = clickedPath === "/" ? "/" : "/" + clickedPath;

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
                const fullPath = clickedPath === "/" ? "" : "/" + clickedPath;

                router.push(fullPath);
              }}
            >
              {"/"}
            </span>
          </div>
        ))}
      </div>
      <div className={styles.right}>
        <div className={styles.spinner} style={{ opacity: loading ? 1 : 0 }}>
          <Spinner />
        </div>
      </div>
    </div>
  );
}
