import clsx from 'clsx';
import { MouseEvent, useEffect, useRef } from 'react';
import { motion } from 'framer-motion';
import { CrossIcon } from '@/assets/images';
import { variantsOverlay, variantsPanel } from '@/components/ui/modal/Modal.variants';
import { Button } from '../button';

import styles from './Modal.module.scss';

type Props = React.PropsWithChildren & {
  heading: string;
  className?: {
    modal?: string;
    wrapper?: string;
  };
  onClose: () => void;
  closeOnMissclick?: boolean;
  showModalMode?: boolean;
};

export function Modal({ heading, children, className, onClose, closeOnMissclick = true, showModalMode = true }: Props) {
  const ref = useRef<HTMLDialogElement>(null);

  const disableScroll = () => document.body.classList.add('modal-open');
  const enableScroll = () => document.body.classList.remove('modal-open');

  const open = () => {
    if (showModalMode) {
      ref.current?.showModal();
    } else {
      ref.current?.show();
    }
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

  const handleClick = ({ target }: MouseEvent) => {
    if (closeOnMissclick) {
      const isBackdropClick = target === ref.current;

      if (isBackdropClick) onClose();
    }
  };

  return (
    <motion.dialog
      initial="enter"
      animate="center"
      exit="exit"
      variants={variantsOverlay}
      ref={ref}
      onClick={handleClick}
      className={clsx(styles.modal, className?.modal)}>
      <motion.div
        initial="enter"
        animate="center"
        exit="exit"
        variants={variantsPanel}
        className={clsx(styles.wrapper, className?.wrapper)}>
        <div className={styles.header}>
          <h2 className={styles.title}>{heading}</h2>

          <Button variant="text" onClick={onClose} className={styles['modal-close']}>
            <CrossIcon />
          </Button>
        </div>

        {children}
      </motion.div>
    </motion.dialog>
  );
}
