import { useState } from 'react';
import { AnimatePresence, motion } from 'framer-motion';
import { Dialog } from '@headlessui/react';
import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@/ui';
import styles from './SearchModal.module.scss';
import { variantsOverlay, variantsPanel } from '../Modal/modal.variants';
import crossIcon from '@/assets/icons/cross-icon.svg';
import { MobileMenuDialogProps } from './SearchModal.interfaces';
import { cx } from '@/utils';
import { Search } from '../Search/Search';

export function SearchModalDialog({ setOpen, open }: MobileMenuDialogProps) {
  const { account } = useAccount();

  return (
    <AnimatePresence initial={false}>
      {open && (
        <Dialog
          as={motion.div}
          initial="closed"
          animate="open"
          exit="closed"
          static
          className={styles.modal}
          open={open}
          onClose={setOpen}>
          <motion.div variants={variantsOverlay} className={styles.modal__backdrop} />
          <div className={styles.modal__wrapper}>
            <div className={styles.modal__container}>
              <Dialog.Panel as={motion.div} variants={variantsPanel} className={styles.modal__content}>
                <Search />
                <Button
                  variant="icon"
                  onClick={() => setOpen(false)}
                  className={styles.modal__close}
                  icon={crossIcon}
                />
              </Dialog.Panel>
            </div>
          </div>
        </Dialog>
      )}
    </AnimatePresence>
  );
}
