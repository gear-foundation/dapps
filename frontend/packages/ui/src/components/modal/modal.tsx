import { useEffect, useRef, MouseEvent, useCallback } from 'react';
import { ReactComponent as CrossSVG } from '../../assets/cross-icon.svg';
import styles from './modal.module.css';
import { ModalProps } from './modal.interface';
import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';

function Modal({ heading, children, onClose, className }: ModalProps) {
  const ref = useRef<HTMLDialogElement>(null);

  const disableScroll = () => document.body.classList.add('modal-open');
  const enableScroll = () => document.body.classList.remove('modal-open');

  const open = useCallback(() => {
    ref.current?.showModal();
    disableScroll();
  }, []);

  const close = useCallback(() => {
    ref.current?.close();
    enableScroll();
  }, []);

  useEffect(() => {
    open();

    return () => close();
  }, [open, close]);

  const handleClick = ({ target }: MouseEvent) => {
    const isBackdropClick = target === ref.current;

    if (isBackdropClick) {
      onClose?.();
    }
  };

  return (
    <dialog ref={ref} onClick={handleClick} className={clsx(styles.modal, className || '')}>
      <div className={clsx(styles.wrapper)}>
        <header className={clsx(styles.header)}>
          <h2>{heading}</h2>
          {onClose && <Button size="small" text="" icon={CrossSVG} onClick={onClose} />}
        </header>
        {children}
      </div>
    </dialog>
  );
}

export { Modal };
