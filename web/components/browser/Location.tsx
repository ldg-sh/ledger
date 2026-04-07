"use client";

import { usePathname, useSearchParams } from "next/navigation";
import styles from "./Location.module.scss";
import { useRouter } from "next/navigation";
import { useRef } from "react";
import Spinner from "../svg/Spinner";
import { useLoading } from "@/context/LoadingContext";
import { useFile } from "@/context/FileExplorerContext";

export default function Location() {
  const fileContext = useFile();
  const { loading } = useLoading();
  const scrollRef = useRef<HTMLDivElement>(null);

  return (
    <div className={styles.locationBar} ref={scrollRef}>
      <div className={styles.left}>
        <span
          className={styles.pathSegment}
          onClick={() => {
            fileContext.gotoPath("");
          }}
        >
          {"home"}
        </span>
        <span className={styles.seperator}>{" / "}</span>
        {(fileContext.breadcrumbs || []).map((_, index) => (
          <div className={styles.pathGrouping} key={index + "-container"}>
            <span
              key={index}
              className={styles.pathSegment}
              onClick={() => {
                fileContext.gotoPath(fileContext.breadcrumbs[index].id);
              }}
            >
              {decodeURIComponent(fileContext.breadcrumbs[index].name)}
            </span>
            <span
              key={index + "-sep"}
              className={styles.seperator}
              onClick={() => {
                fileContext.gotoPath(fileContext.breadcrumbs[index].id);
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
