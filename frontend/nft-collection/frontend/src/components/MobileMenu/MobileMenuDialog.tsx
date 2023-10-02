import { useState } from 'react';
import { useAtomValue } from 'jotai';
import { AnimatePresence, motion } from 'framer-motion';
import { Dialog } from '@headlessui/react';
import { useAccount } from '@gear-js/react-hooks';
import { Button, Link } from '@/ui';
import { ADDRESS } from '@/consts';
import { useAuth } from '@/features/Auth/hooks';
import { AccountIcon } from '@/features/Wallet/components/account-icon';
import { WalletModal } from '@/features/Wallet/components';
import { ReactComponent as VaraSignIcon } from '@/assets/icons/vara-coin.svg';
import { ReactComponent as EditIcon } from '@/assets/icons/ic-edit-24.svg';
import styles from './MobileMenu.module.scss';
import { variantsOverlay, variantsPanel } from '../Modal/modal.variants';
import crossIcon from '@/assets/icons/cross-icon.svg';
import { ReactComponent as ContractAddressIcon } from '@/assets/icons/contract-address-rounded.svg';
import { ReactComponent as ChangeAccountIcon } from '@/assets/icons/change-account.svg';
import { ReactComponent as DisconnectIcon } from '@/assets/icons/disconnect.svg';
import { MobileMenuDialogProps } from './MobileMenu.interfaces';
import { useNodes } from '@/features/NodeSwitch/hooks';
import { cx, shortenString } from '@/utils';
import { ResultNode } from '@/features/NodeSwitch/types';

import { CONTRACT_ADDRESS_ATOM } from '@/atoms';
import { EXPLORE, YOUR_SPACE } from '@/routes';

function SwitchAccount({ onClose }: { onClose(): void }) {
  const [openWallet, setOpenWallet] = useState(false);
  return (
    <>
      <ChangeAccountIcon onClick={() => setOpenWallet(true)} />
      <WalletModal open={openWallet} setOpen={setOpenWallet} onClose={onClose} />
    </>
  );
}

export function MobileMenuDialog({ setOpen, open }: MobileMenuDialogProps) {
  const { account } = useAccount();
  const { signOut } = useAuth();
  const { nodeSections } = useNodes();
  const contractAddress = useAtomValue(CONTRACT_ADDRESS_ATOM);
  const currentNode = nodeSections
    ?.reduce((acc: ResultNode[], section) => {
      const sectionNodes = section.nodes.map((node) => ({
        caption: section.caption,
        address: node.address,
      }));

      return [...acc, ...sectionNodes];
    }, [])
    ?.find((n) => n.address === ADDRESS.NODE);

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
                  <ul className={cx(styles.options)}>
                    <Link to={EXPLORE} onClick={() => setOpen(false)}>
                      <li className={cx(styles.option)}>Explore NFTs & Collections</li>
                    </Link>
                    <Link to={YOUR_SPACE} onClick={() => setOpen(false)}>
                      <li className={cx(styles.option)}>Your Space</li>
                    </Link>
                  </ul>
                </div>
                <div className={styles.item}>
                  <Button
                    variant="primary"
                    label="Create Collection"
                    size="medium"
                    className={cx(styles['create-btn'])}
                  />
                </div>
                <div className={cx(styles.item, styles['item-with-border-top'], styles['item-splited'])}>
                  <ContractAddressIcon width={32} height={32} />
                  <div className={styles.item__text}>
                    <p className={styles.item__title}>Contract Address</p>
                    <p className={styles.item__helper}>{shortenString(contractAddress || '', 8)}</p>
                  </div>
                  <EditIcon />
                </div>
                <div className={cx(styles.item, styles['item-with-border-top'], styles['item-splited'])}>
                  <VaraSignIcon width={32} height={32} />
                  <div className={styles.item__text}>
                    <p className={styles.item__title}>{currentNode?.caption}</p>
                    <p className={styles.item__helper}>{currentNode?.address}</p>
                  </div>
                  <EditIcon />
                </div>
                <div className={cx(styles.item, styles['item-with-border-top'], styles['item-splited'])}>
                  <AccountIcon value={account?.address} size={32} />
                  <div className={styles.item__text}>
                    <p className={styles.item__title}>{account?.meta.name}</p>
                  </div>
                  <div className={styles.actions}>
                    <SwitchAccount onClose={() => setOpen(false)} />
                    <DisconnectIcon onClick={() => signOut()} />
                  </div>
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
