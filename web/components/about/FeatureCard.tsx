import styles from "./FeatureCard.module.scss";

export default function FeatureCard({
  title,
  description,
  icon,
}: {
  title: string;
  description: string;
  icon: React.ReactNode;
}) {
  return (
    <div className={styles.featureCard}>
      <div className={styles.titleContainer}>
        <div className={styles.icon}>{icon}</div>
        <h2 className={styles.title}>{title}</h2>
      </div>
      <p className={styles.description}>{description}</p>
    </div>
  );
}
