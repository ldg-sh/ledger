import { createPortal } from 'react-dom';
import styles from './ContextMenu.module.scss';
import { motion } from 'motion/react';

interface MenuProps {
  x: number;
  y: number;
  children: React.ReactNode;
}

export const ContextMenu = ({ x, y, children }: MenuProps) => {
  return createPortal(
    <motion.div 
      style={{ top: y, left: x }}
      className={styles.contextMenu}
      initial={{ opacity: 0, transform: 'translateY(10px)', transition: { duration: 0.1 } }}
      animate={{ opacity: 1, transform: 'translateY(0)', transition: { duration: 0.1 } }}
      exit={{ opacity: 0, transform: 'translateY(-10px)', transition: { duration: 0.1 } }}
    >
      {children}
    </motion.div>,
    document.body
  );
};