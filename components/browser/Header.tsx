import styles from "./header.module.scss";

export default function Header() {
    return (
        <header>
            <div className={styles.spacer}></div>
            <span className={styles.fileName + " " + styles.headerElement}>File Name</span>
            <span className={styles.fileSize + " " + styles.headerElement}>Size</span>
            <span className={styles.fileType + " " + styles.headerElement}>Type</span>
            <span className={styles.createdAt + " " + styles.headerElement}>Date Created</span>
        </header>
    );
}