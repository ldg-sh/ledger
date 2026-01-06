import { Icon, Square } from 'lucide-react';
import { DynamicIcon } from 'lucide-react/dynamic';
import styles from './row.module.scss';
import { getFileIcon } from '@/lib/util/icon';

interface RowProps {
    fileName: string;
    fileSize: number;
    fileType: string;
    createdAt: string;
}

export default function Row(
    { fileName, fileSize, fileType, createdAt }: RowProps
) {
    let date = new Date(createdAt);
    let options: Intl.DateTimeFormatOptions = { year: 'numeric', month: 'long', day: 'numeric' };
    let formattedDate = date.toLocaleDateString(undefined, options);

    let sizeUnit = 'B';
    let displaySize = fileSize;
    if (fileSize >= 1024) {
        displaySize = fileSize / 1024;
        sizeUnit = 'KB';
    }
    if (displaySize >= 1024) {
        displaySize = displaySize / 1024;
        sizeUnit = 'MB';
    }
    if (displaySize >= 1024) {
        displaySize = displaySize / 1024;
        sizeUnit = 'GB';
    }

    return (
        <div className={styles.row}>
            <Square size={16} strokeWidth={1.6} color={"var(--color-text-secondary)"} className={styles.rowElement} />
            <DynamicIcon name={getFileIcon(fileType) as any} size={16} strokeWidth={1.6} color={"var(--color-text-secondary)"} className={styles.rowElement} />
            <span className={styles.fileName + " " + styles.rowElement}>{fileName}</span>
            <span className={styles.fileSize + " " + styles.rowElement}>{displaySize.toFixed(1)} {sizeUnit}</span>
            <span className={styles.fileType + " " + styles.rowElement}>{fileType}</span>
            <span className={styles.createdAt + " " + styles.rowElement}>{formattedDate}</span>
        </div>
    );
}