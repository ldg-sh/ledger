import { cn } from "@/lib/util/class";
import styles from "./Spinner.module.scss";

interface SpinnerProps {
  height?: number;
  destructive?: boolean;
}

export default function Spinner({ height, destructive }: SpinnerProps) {
  return (
    <svg
      width={height || 20}
      height={height || 20}
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      className={styles.container}
    >
      <circle
        cx="12" cy="12" r="9.5"
        fill="none"
        stroke="var(--color-framework)"
        strokeWidth="1.5"
        className={cn(destructive && styles.destructiveBackground)}
      />
      <circle
        cx="12" cy="12" r="9.5"
        fill="none"
        stroke="var(--color-text-primary)"
        strokeWidth="1.5"
        strokeLinecap="round"
        strokeDasharray="15 44.7"
        className={cn(styles.spinner, destructive && styles.destructive)}
      />
    </svg>
  );
}
