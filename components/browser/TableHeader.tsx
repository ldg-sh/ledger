import { cn } from "@/lib/util/class";
import styles from "./TableHeader.module.scss";

export default function TableHeader() {
    return (
        <header className={styles.tableHeader}>
            <div className={styles.spacer}></div>
            <span className={cn(styles.fileName, styles.headerElement)}>Name</span>
            <span className={cn(styles.fileSize, styles.headerElement)}>Size</span>
            <span className={cn(styles.fileType, styles.headerElement)}>Type</span>
            <span className={cn(styles.createdAt, styles.headerElement)}>Date Created</span>
        </header>
    );
}