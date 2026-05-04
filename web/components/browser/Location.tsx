"use client";

import { useFile } from "@/context/FileExplorerContext";
import { useEffect, useRef, useState } from "react";
import styles from "./Location.module.scss";

export default function Location() {
  const fileContext = useFile();
  const scrollRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  const [displayValue, setDisplayValue] = useState(fileContext.searchQuery);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setDisplayValue(event.target.value);
  };

  useEffect(() => {
    const handler = setTimeout(() => {
      fileContext.setSearchQuery(displayValue);
    }, 200);

    return () => {
      clearTimeout(handler);
    };
  }, [displayValue, fileContext]);

  return (
    <div className={styles.locationBar} ref={scrollRef}>
      {fileContext.searchQuery ? (
        <span className={styles.searchQuery}>
          Searching <strong>{fileContext.searchQuery}</strong> globally...
        </span>
      ) : (
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
      )}

      <div className={styles.search} onClick={() => inputRef.current?.focus()}>
        <svg
          width="16px"
          height="16px"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            d="M15.7955 15.8111L21 21M18 10.5C18 14.6421 14.6421 18 10.5 18C6.35786 18 3 14.6421 3 10.5C3 6.35786 6.35786 3 10.5 3C14.6421 3 18 6.35786 18 10.5Z"
            stroke="var(--color-text-secondary)"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          />
        </svg>
        <input
          type="text"
          ref={inputRef}
          value={displayValue}
          onChange={handleChange}
          placeholder="Search..."
        />
      </div>
    </div>
  );
}
