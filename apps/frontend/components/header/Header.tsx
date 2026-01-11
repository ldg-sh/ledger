import styles from "./Header.module.scss";

export default function Logo() {
  return (
    <div className={styles.header}>
      <div className={styles.headerCenter}>
        <div className={styles.logo}>
          <svg
            width="35"
            height="35"
            viewBox="0 0 514 514"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M308.906 43.0439H357.015V156.985H465.892V200.029H514V357.015H205.094V514H43.0439V465.892H0V308.906H156.985V351.951H200.029V308.906H156.985V0H308.906V43.0439ZM156.985 465.892H48.1084V508.936H200.029V357.015H156.985V465.892ZM465.892 308.906H205.094V351.951H508.936V205.094H465.892V308.906ZM308.906 156.985H351.951V48.1084H308.906V156.985Z"
              fill="var(--color-text-bold)"
            />
          </svg>
          <div className={styles.logoText}>
            <h1 className={styles.logoTitle}>Ledger</h1>
            <p className={styles.logoSubtitle}>
              Fast, efficient, lightweight file storage.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
