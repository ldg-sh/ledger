"use client";
import { useFile } from "@/context/FileExplorerContext";
import { useLoading } from "@/context/LoadingContext";
import { useSort } from "@/context/SortContext";
import { cn } from "@/lib/util/class";
import { ChevronDown, ChevronsUpDown, ChevronUp } from "lucide-react";
import Spinner from "../svg/Spinner";
import styles from "./TableHeader.module.scss";

const SortIcon = ({ category, sort }: { category: string; sort: string }) => {
  const isActive = sort.startsWith(category);
  const isAsc = sort.endsWith("_asc");

  if (!isActive) {
    return <ChevronsUpDown className={styles.sortIcon} size={14} />;
  }

  return isAsc ? (
    <ChevronUp className={styles.sortIconActive} size={14} />
  ) : (
    <ChevronDown className={styles.sortIconActive} size={14} />
  );
};

export default function TableHeader() {
  const { sort, toggleSort } = useSort();
  const { loading } = useLoading();
  const { searchQuery } = useFile();

  return (
    <header className={styles.tableHeader}>
      <div className={styles.spacer}></div>

      <button
        className={cn(styles.headerElement, styles.fileName)}
        style={{
          opacity: searchQuery ? 0.5 : 1,
          pointerEvents: searchQuery !== "" ? "none" : "auto",
          cursor: searchQuery !== "" ? "not-allowed" : "pointer",
        }}
        disabled={searchQuery !== ""}
        onClick={() => toggleSort("name")}
      >
        <span>Name</span>
        <SortIcon category="name" sort={sort} />
      </button>

      <button
        className={cn(styles.headerElement, styles.fileSize)}
        style={{
          opacity: searchQuery ? 0.5 : 1,
          pointerEvents: searchQuery !== "" ? "none" : "auto",
          cursor: searchQuery !== "" ? "not-allowed" : "pointer",
        }}
        disabled={searchQuery !== ""}
        onClick={() => toggleSort("size")}
      >
        <span>Size</span>
        <SortIcon category="size" sort={sort} />
      </button>

      <button
        className={cn(styles.headerElement, styles.fileType)}
        style={{
          opacity: searchQuery ? 0.5 : 1,
          pointerEvents: searchQuery !== "" ? "none" : "auto",
          cursor: searchQuery !== "" ? "not-allowed" : "pointer",
        }}
        disabled={searchQuery !== ""}
        onClick={() => toggleSort("type")}
      >
        <span>Type</span>
        <SortIcon category="type" sort={sort} />
      </button>

      <button
        className={cn(styles.headerElement, styles.createdAt)}
        style={{
          opacity: searchQuery ? 0.5 : 1,
          pointerEvents: searchQuery !== "" ? "none" : "auto",
          cursor: searchQuery !== "" ? "not-allowed" : "pointer",
        }}
        disabled={searchQuery !== ""}
        onClick={() => toggleSort("date")}
      >
        <span>Date Created</span>
        <SortIcon category="date" sort={sort} />
      </button>

      <div
        className={styles.spinner}
        style={{
          opacity: loading ? 1 : 0,
        }}
      >
        <Spinner />
      </div>
    </header>
  );
}
