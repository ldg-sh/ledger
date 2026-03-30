import Link from "next/link";
import styles from "./GetStarted.module.scss";

export default function GetStarted() {
  return <Link className={styles.getStartedButton} href="/login">
    Get Started
  </Link>;
}
