import { motion } from 'framer-motion';
import React, { useEffect, useRef, MouseEventHandler } from 'react';

import { CrossIcon } from '@/assets/images';
import { Button } from '@/components/ui/button';

import { ScrollArea } from '../scroll-area';

import styles from './Modal.module.scss';

type Props = React.PropsWithChildren & {
  heading: string;
  onClose: () => void;
};

export function ModalBottom({ heading, children, onClose }: Props) {
  const ref = useRef<HTMLDialogElement>(null);

  const disableScroll = () => document.body.classList.add('modal-bottom-open');
  const enableScroll = () => document.body.classList.remove('modal-bottom-open');

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

  const handleClick: MouseEventHandler<HTMLDialogElement> = (event) => {
    if (ref.current === event.target) {
      onClose();
    }
  };

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        onClose();
      }
    };

    document.addEventListener('keydown', handleKeyDown);

    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [onClose]);

  return (
    <motion.dialog ref={ref} onClick={handleClick} className={styles.modal}>
      <motion.div className={styles.wrapper}>
        <div className={styles.header}>
          <h2 className={styles.title}>{heading}</h2>
          <Button variant="text" onClick={onClose} className={styles['modal-close']}>
            <CrossIcon />
          </Button>
        </div>
        <ScrollArea type="auto" className={styles.scroll}>
          {children}
        </ScrollArea>
      </motion.div>
    </motion.dialog>
  );
}
