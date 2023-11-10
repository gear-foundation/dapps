import { Button } from '@/components/ui/button';
import styles from './mobile-menu.module.scss';
import { useState } from 'react';
import { ADDRESS } from '@/app/consts';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { WalletIcon } from '@/features/wallet';
import { AnimatePresence, motion } from 'framer-motion';
import { variantsOverlay, variantsPanel } from '@/components/ui/modal/modal.variants';
import { Dialog } from '@headlessui/react';
import { useAuth } from '@/features/auth';
import { DialogsLibrary } from '@/components/ui/dialogs';
import { Sprite } from '@/components/ui/sprite';
import { useGame } from '@/features/tic-tac-toe/hooks';

export type MobileMenuDialogProps = {
  onClose?(): void;
  open: boolean;
  setOpen(value: boolean): void;
};

export function MobileMenuDialog({ setOpen, open }: MobileMenuDialogProps) {
  const { api } = useApi();
  const { account } = useAccount();
  const { signOut } = useAuth();
  const { resetGame } = useGame();

  const handleLogoutButtonClick = () => {
    signOut();
    setOpen(false);
    resetGame();
  };

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
                <div className={styles.item}>
                  <div className={styles.item__icon}>
                    <Sprite name="vara-sign" size={16} />
                  </div>
                  <div className={styles.item__text}>
                    <p className={styles.item__title}>{api?.runtimeVersion.specName.toHuman()}</p>
                    <p className={styles.item__helper}>{ADDRESS.NODE}</p>
                  </div>
                </div>
                <hr />
                <div className={styles.item}>
                  <WalletIcon address={account?.address} size={32} />
                  <div className={styles.item__text}>
                    <p className={styles.item__title}>{account?.meta.name}</p>
                  </div>
                </div>
                <div className={styles.actions}>
                  <SwitchAccount onClose={() => setOpen(false)} />
                  <Button variant="black" onClick={handleLogoutButtonClick}>
                    Disconnect
                  </Button>
                </div>
                <motion.div
                  variants={{
                    opened: { opacity: 1, transition: { delay: 0.3 } },
                    closed: { opacity: 0 },
                  }}>
                  <Button variant="text" onClick={() => setOpen(false)} className={styles.modal__close}>
                    <Sprite name="close" width={25} height={24} />
                  </Button>
                </motion.div>
              </Dialog.Panel>
            </div>
          </div>
        </Dialog>
      )}
    </AnimatePresence>
  );
}

function SwitchAccount({ onClose }: { onClose(): void }) {
  const [openWallet, setOpenWallet] = useState(false);
  return (
    <>
      <Button onClick={() => setOpenWallet(true)}>Change account</Button>
      <DialogsLibrary.WalletModal open={openWallet} setOpen={setOpenWallet} onClose={onClose} />
    </>
  );
}
