import { ReactNode, useEffect, useRef, MouseEvent } from 'react';
import { ReactComponent as CrossSVG } from 'assets/images/icons/cross.svg';
import clsx from 'clsx';
import styles from './Modal.module.scss';
import { Button } from '../button';

type Props = {
  heading?: string;
  children: ReactNode;
  onClose: () => void;
  className?: string;
};

function Modal({ heading, children, onClose, className }: Props) {
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
    // eslint-disable-next-line jsx-a11y/click-events-have-key-events, jsx-a11y/no-noninteractive-element-interactions
    <dialog ref={ref} onClick={handleClick} className={clsx(styles.modal, className)}>
      <div className={styles.wrapper}>
        <header className={styles.header}>
          {heading && <h2 className={styles.title}>{heading}</h2>}

          <Button variant="text" onClick={onClose} className={styles['modal-close']}>
            <CrossSVG />
          </Button>
        </header>

        {children}
      </div>
    </dialog>
  );
}

export { Modal };
