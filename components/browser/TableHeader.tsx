import styles from "./tableheader.module.scss";

export default function TableHeader() {
    return (
        <header className={styles.tableHeader}>
            <div className={styles.spacer}></div>
            <span className={styles.fileName + " " + styles.headerElement}>Name</span>
            <span className={styles.fileSize + " " + styles.headerElement}>Size</span>
            <span className={styles.fileType + " " + styles.headerElement}>Type</span>
            <span className={styles.createdAt + " " + styles.headerElement}>Date Created</span>
        </header>
    );
}