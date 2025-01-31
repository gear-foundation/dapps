import { MouseEvent, useEffect, useRef } from 'react';
import clsx from 'clsx';
import { motion } from 'framer-motion';
import { ReactComponent as CrossIcon } from '@/assets/images/icons/cross-icon.svg';
import { variantsOverlay, variantsPanel } from '@/components/layout/modal/Modal.variants';
import { Button } from '@gear-js/vara-ui';

import styles from './Modal.module.scss';

type Props = React.PropsWithChildren & {
  heading: string;
  className?: {
    header?: string;
  };
  onClose: () => void;
};

export function Modal({ heading, children, onClose, className }: Props) {
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
      <motion.div initial="enter" animate="center" exit="exit" variants={variantsPanel} className={styles.wrapper}>
        <div className={clsx(styles.header, className?.header)}>
          <h2 className={styles.title}>{heading}</h2>

          <Button onClick={onClose} color="transparent" className={styles['modal-close']}>
            <CrossIcon />
          </Button>
        </div>

        {children}
      </motion.div>
    </motion.dialog>
  );
}
