import { VariantProps, cva } from 'class-variance-authority';
import clsx from 'clsx';
import { motion } from 'framer-motion';
import { MouseEvent, useEffect, useRef } from 'react';

import type { BaseComponentProps } from '@/app/types';
import { variantsOverlay, variantsPanel } from '@/components/ui/modal/modal.variants';
import { Sprite } from '@/components/ui/sprite';

import { Button } from '../button';

import styles from './Modal.module.scss';

const variants = cva('', {
  variants: { size: { md: styles.md, sm: styles.sm } },
  defaultVariants: { size: 'md' },
});

type Props = BaseComponentProps &
  VariantProps<typeof variants> & {
    title: string;
    description?: string;
    onClose: () => void;
    buttons?: React.ReactNode;
    modalClassName?: string;
    closeOnBackdrop?: boolean;
  };

export function Modal({
  title,
  description,
  buttons,
  onClose,
  className,
  modalClassName,
  closeOnBackdrop = true,
  size,
  children,
}: Props) {
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

    if (isBackdropClick && closeOnBackdrop) onClose();
  };

  return (
    // We don't use ::backdrop for displaing alerts
    <div className={styles.backdrop}>
      <motion.dialog
        initial="enter"
        animate="center"
        exit="exit"
        variants={variantsOverlay}
        ref={ref}
        onClick={handleClick}
        className={variants({ className: clsx(styles.modal, modalClassName), size })}>
        <motion.div
          initial="enter"
          animate="center"
          exit="exit"
          variants={variantsPanel}
          className={clsx(styles.wrapper, className)}>
          <div className={styles.header}>
            <div className={styles.titleContainer}>
              <h2 className={styles.title}>{title}</h2>
              {description && <p className={styles.description}>{description}</p>}
            </div>
            <Button variant="text" onClick={onClose} className={styles['modal-close']}>
              <Sprite name="close" width={25} height={24} />
            </Button>
          </div>

          {children}

          {buttons && <div className={styles.buttons}>{buttons}</div>}
        </motion.div>
      </motion.dialog>
    </div>
  );
}
