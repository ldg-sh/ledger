"use client";

import Button from '@/components/general/Button';
import styles from './not-found.module.scss';
import { useUser } from '@/context/UserContext';

export default function NotFound() {
  const { user } = useUser();

  return (
    <div className={styles.notFound}>
      <div className={styles.content}>
        <h1 className={styles.status}>404</h1>
        <p className={styles.message}>The page or file you are looking for does not exist.</p>
        <Button width='150px' label="Go Home" href={user ? '/' : '/about'} />
      </div>
      <div className={styles.background}>
        <svg
          width="100%"
          height="100%"
          viewBox="0 0 514 514"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          className={styles.svg}
        >
          <path
            d="M308.906 43.0439H357.015V156.985H465.892V200.029H514V357.015H205.094V514H43.0439V465.892H0V308.906H156.985V351.951H200.029V308.906H156.985V0H308.906V43.0439ZM156.985 465.892H48.1084V508.936H200.029V357.015H156.985V465.892ZM465.892 308.906H205.094V351.951H508.936V205.094H465.892V308.906ZM308.906 156.985H351.951V48.1084H308.906V156.985Z"
          />
        </svg>
      </div>
    </div>
  );
}
