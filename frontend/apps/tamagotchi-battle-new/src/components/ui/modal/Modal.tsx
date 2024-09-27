import clsx from 'clsx';
import { MouseEvent, useEffect, useRef } from 'react';
import { motion } from 'framer-motion';
import { variantsOverlay, variantsPanel } from '@/components/ui/modal/modal.variants';
import { Button } from '../button';
import { Sprite } from '@/components/ui/sprite';
import type { BaseComponentProps } from '@/app/types';
import styles from './Modal.module.scss';

type Props = BaseComponentProps & {
  title: string;
  description?: string;
  onClose: () => void;
  buttons?: React.ReactNode;
};

export function Modal({ title, description, children, buttons, onClose, className }: Props) {
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

  const handleClick = ({ target }: MouseEvent) => {
    const isBackdropClick = target === ref.current;

    if (isBackdropClick) onClose();
  };

  return (
    <motion.dialog
      initial="enter"
      animate="center"
      exit="exit"
      variants={variantsOverlay}
      ref={ref}
      onClick={handleClick}
      className={styles.modal}>
      <motion.div
        initial="enter"
        animate="center"
        exit="exit"
        variants={variantsPanel}
        className={clsx(styles.wrapper, className)}>
        <div className={styles.header}>
          <div className={styles.titleContainer}>
            <h2 className={styles.title}>{title}</h2>
            <p className={styles.description}>{description}</p>
          </div>
          <Button variant="text" onClick={onClose} className={styles['modal-close']}>
            <Sprite name="close" width={25} height={24} />
          </Button>
        </div>

        {children}

        {buttons && <div className={styles.buttons}>{buttons}</div>}
      </motion.div>
    </motion.dialog>
  );
}
