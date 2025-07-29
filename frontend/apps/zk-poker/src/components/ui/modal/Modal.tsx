import clsx from 'clsx';
import { motion } from 'framer-motion';
import { MouseEvent, useEffect, useRef, forwardRef } from 'react';

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
  onClose?: () => void;
  closeOnMissclick?: boolean;
  // hacky fix cuz the signless modal was not displaying above the dialog opened via showModal
  showModalMode?: boolean;
  isDark?: boolean;
};

export const Modal = forwardRef<HTMLDialogElement, Props>(
  ({ heading, children, className, onClose, closeOnMissclick = true, showModalMode = true, isDark = false }, ref) => {
    const localRef = useRef<HTMLDialogElement>(null);
    const dialogRef = (ref || localRef) as React.RefObject<HTMLDialogElement>;

    const disableScroll = () => document.body.classList.add('modal-open');
    const enableScroll = () => document.body.classList.remove('modal-open');

    const open = () => {
      if (showModalMode) {
        dialogRef.current?.showModal();
      } else {
        dialogRef.current?.show();
      }
      disableScroll();
    };

    const close = () => {
      dialogRef.current?.close();
      enableScroll();
    };

    useEffect(() => {
      open();

      return () => close();
      // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    const handleClick = ({ target }: MouseEvent) => {
      if (closeOnMissclick) {
        const isBackdropClick = target === dialogRef.current;

        if (isBackdropClick && onClose) onClose();
      }
    };

    return (
      <motion.dialog
        initial="enter"
        animate="center"
        exit="exit"
        variants={variantsOverlay}
        ref={dialogRef}
        onClick={handleClick}
        className={clsx(styles.modal, className?.modal, isDark && styles.dark)}>
        <motion.div
          initial="enter"
          animate="center"
          exit="exit"
          variants={variantsPanel}
          className={clsx(styles.wrapper, className?.wrapper)}>
          <div className={styles.header}>
            <h2 className={styles.title}>{heading}</h2>

            {onClose && (
              <Button color="transparent" onClick={onClose} className={styles['modal-close']}>
                <CrossIcon />
              </Button>
            )}
          </div>

          {children}
        </motion.div>
      </motion.dialog>
    );
  },
);

Modal.displayName = 'Modal';
