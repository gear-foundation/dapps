import { useState } from 'react';
import { AnimatePresence, motion } from 'framer-motion';
import { Dialog } from '@headlessui/react';
import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@/ui';
import { ADDRESS } from '@/consts';
import { useAuth } from '@/features/Auth/hooks';
import { AccountIcon } from '@/features/Wallet/components/account-icon';
import { WalletModal } from '@/features/Wallet/components';
import { ReactComponent as VaraSignIcon } from '@/assets/icons/vara-sign.svg';
import styles from './MobileMenu.module.scss';
import { variantsOverlay, variantsPanel } from '../Modal/modal.variants';
import crossIcon from '@/assets/icons/cross-icon.svg';
import { MobileMenuDialogProps } from './MobileMenu.interfaces';
import { useNodes } from '@/hooks';
import { cx } from '@/utils';

function SwitchAccount({ onClose }: { onClose(): void }) {
  const [openWallet, setOpenWallet] = useState(false);
  return (
    <>
      <Button onClick={() => setOpenWallet(true)} label="Change account" variant="primary" className={cx(styles.btn)} />
      <WalletModal open={openWallet} setOpen={setOpenWallet} onClose={onClose} />
    </>
  );
}

export function MobileMenuDialog({ setOpen, open }: MobileMenuDialogProps) {
  const { account } = useAccount();
  const { signOut } = useAuth();
  const { nodes } = useNodes();
  const currentNode = nodes?.find((n) => n.address === ADDRESS.NODE);

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
                  <VaraSignIcon width={32} height={32} />
                  <div className={styles.item__text}>
                    <p className={styles.item__title}>{currentNode?.caption}</p>
                    <p className={styles.item__helper}>{currentNode?.address}</p>
                  </div>
                </div>
                <hr />
                <div className={styles.item}>
                  <AccountIcon value={account?.address} size={32} />
                  <div className={styles.item__text}>
                    <p className={styles.item__title}>{account?.meta.name}</p>
                  </div>
                </div>
                <div className={styles.actions}>
                  <SwitchAccount onClose={() => setOpen(false)} />
                  <Button
                    variant="primary"
                    onClick={() => signOut()}
                    label="Disconnect"
                    className={cx(styles.btn, styles['btn-disconnect'])}
                  />
                </div>
                <motion.div
                  variants={{
                    opened: { opacity: 1, transition: { delay: 0.3 } },
                    closed: { opacity: 0 },
                  }}>
                  <Button
                    variant="icon"
                    onClick={() => setOpen(false)}
                    className={styles.modal__close}
                    icon={crossIcon}
                  />
                </motion.div>
              </Dialog.Panel>
            </div>
          </div>
        </Dialog>
      )}
    </AnimatePresence>
  );
}
