import GetStarted from "@/components/about/GetStarted";
import styles from "./page.module.scss";
import User from "@/components/header/user/User";
import FeatureCard from "@/components/about/FeatureCard";
import { cn } from "@/lib/util/class";

export default function AboutPage() {
  return (
    <div className={styles.aboutContainer}>
      <div className={styles.topBar}>
        <User />
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
            fill="#f3f3f3"
          />
        </svg>
      </div>
      <div className={styles.topContainer}>
        <div className={styles.divider} />
      </div>
      <div className={styles.text}>
        <h1 className={styles.title}>Ledger</h1>
        <p className={styles.descriptionBold}>
          Fast and efficient file storage.
        </p>
        <p className={styles.description}>
          Web storage at the edge, providing you with the fastest possible
          upload and download speeds in a simplistic interface.
        </p>
        <div className={styles.buttonContainer}>
          <GetStarted />
        </div>
      </div>
      <div className={styles.bottomContainer}>
        <div className={cn(styles.features)}>
          <FeatureCard
            title="Lighting Fast"
            description="Ledger uses multithreaded edge-based uploading to expedite your transfer at speeds up to 4x that of Google Drive."
            icon={
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="var(--color-text-primary)"
                strokeWidth="2.5"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <path d="M13 16a3 3 0 0 1 2.24 5"></path>
                <path d="M18 12h.01"></path>
                <path d="M18 21h-8a4 4 0 0 1-4-4 7 7 0 0 1 7-7h.2L9.6 6.4a1 1 0 1 1 2.8-2.8L15.8 7h.2c3.3 0 6 2.7 6 6v1a2 2 0 0 1-2 2h-1a3 3 0 0 0-3 3"></path>
                <path d="M20 8.54V4a2 2 0 1 0-4 0v3"></path>
                <path d="M7.612 12.524a3 3 0 1 0-1.6 4.3"></path>
              </svg>
            }
          />
          <FeatureCard
            title="Shareable"
            description="Build to simplify sharing files between your teammates. Click copy link and you’re done."
            icon={
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="var(--color-text-primary)"
                strokeWidth="2.5"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <circle cx="18" cy="5" r="3"></circle>
                <circle cx="6" cy="12" r="3"></circle>
                <circle cx="18" cy="19" r="3"></circle>
                <line x1="8.59" x2="15.42" y1="13.51" y2="17.49"></line>
                <line x1="15.41" x2="8.59" y1="6.51" y2="10.49"></line>
              </svg>
            }
          />
          <FeatureCard
            title="Secure"
            description="Log in with Google, GitHub, or a passkey. Ledger does not store any credentials or personal information."
            icon={
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="var(--color-text-primary)"
                strokeWidth="2.5"
                strokeLinecap="round"
                strokeLinejoin="round"
              >
                <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"></path>
                <path d="m9 12 2 2 4-4"></path>
              </svg>
            }
          />
        </div>
      </div>
      <div className={styles.footer}>
        <div className={styles.innerFooter}>
          <p className={styles.footerText}>
            © {new Date().getFullYear()} Ledger. All rights reserved. Built by{" "}
            <a
              href="https://thesamgordon.com"
              target="_blank"
              rel="noopener noreferrer"
              className={styles.footerLink}
            >
              Sam Gordon
            </a>
            .
          </p>
        </div>
      </div>
    </div>
  );
}
