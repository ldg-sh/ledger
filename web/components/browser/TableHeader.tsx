"use client";
import { useSort } from "@/context/SortContext";
import { cn } from "@/lib/util/class";
import { ChevronUp, ChevronDown, ChevronsUpDown } from "lucide-react";
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

  return (
    <header className={styles.tableHeader}>
      <div className={styles.spacer}></div>

      <button
        className={cn(styles.headerElement, styles.fileName)}
        onClick={() => toggleSort("name")}
      >
        <span>Name</span>
        <SortIcon category="name" sort={sort} />
      </button>

      <button
        className={cn(styles.headerElement, styles.fileSize)}
        onClick={() => toggleSort("size")}
      >
        <span>Size</span>
        <SortIcon category="size" sort={sort} />
      </button>

      <button
        className={cn(styles.headerElement, styles.fileType)}
        onClick={() => toggleSort("type")}
      >
        <span>Type</span>
        <SortIcon category="type" sort={sort} />
      </button>

      <button
        className={cn(styles.headerElement, styles.createdAt)}
        onClick={() => toggleSort("date")}
      >
        <span>Date Created</span>
        <SortIcon category="date" sort={sort} />
      </button>
    </header>
  );
}
