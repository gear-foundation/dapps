import { motion } from 'framer-motion';
import { useEffect, useRef } from 'react';

import type { BaseComponentProps } from '@/app/types';
import { variantsOverlay, variantsPanel } from '@/components/ui/modal/modal.variants';

import styles from './Modal.module.scss';

export function Modal({ children }: BaseComponentProps) {
  const ref = useRef<HTMLDialogElement>(null);

  const disableScroll = () => document.body.classList.add('modal-open');
  const enableScroll = () => document.body.classList.remove('modal-open');

  const open = () => {
    ref.current?.showModal();
    disableScroll();
  };

  const close = () => {
    ref.current?.close();
    enableScroll();
  };

  useEffect(() => {
    open();

    return () => close();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <motion.dialog
      initial="enter"
      animate="center"
      exit="exit"
      variants={variantsOverlay}
      ref={ref}
      className={styles.modal}>
      <motion.div initial="enter" animate="center" exit="exit" variants={variantsPanel} className={styles.wrapper}>
        {children}
      </motion.div>
    </motion.dialog>
  );
}
