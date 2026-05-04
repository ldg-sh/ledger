interface SpinnerProps {
  height?: number;
  destructive?: boolean;
}

export default function Check({ height }: SpinnerProps) {
  return (
    <svg
      width={height || 20}
      height={height || 20}
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M20 6 9 17l-5-5"
        stroke={"var(--color-text-primary)"}
        strokeWidth={3}
        strokeLinecap="round"
      ></path>
    </svg>
  );
}
