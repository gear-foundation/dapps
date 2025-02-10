import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { Wallet, WalletModal } from '@gear-js/wallet-connect';
import clsx from 'clsx';
import { motion } from 'framer-motion';
import { useRef, useState } from 'react';

import CrossSVG from '@ui/assets/cross-icon.svg?react';
import { useClickOutside, useRootModalRef } from '@ui/utils';

import { MenuOptions, MobileMenu } from '..';
import { MenuOptionsClassNameProps } from '../menu-options';
import { MobileMenuClassNameProps } from '../mobile-menu';

import BurgerMenuSVG from './assets/burger-menu.svg?react';
import styles from './menu-handler.module.css';

type Props = {
  customItems?: {
    icon?: React.FunctionComponent<React.SVGProps<SVGSVGElement>>;
    option: JSX.Element;
    key: string;
  }[];
  className?: {
    container?: string;
    dropdown?: string;
    mobileMenuWrapper?: string;
    icon?: string;
    menuOptions?: MenuOptionsClassNameProps;
    mobileMenu?: MobileMenuClassNameProps;
  };
};

export function MenuHandler({ customItems, className }: Props) {
  const menuRef = useRef<HTMLDivElement>(null);
  const { account } = useAccount();

  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const openMenu = () => setIsMenuOpen(true);
  const closeMenu = () => setIsMenuOpen(false);

  const [isWalletModalOpen, setIsWalletModalOpen] = useState(false);
  const openWalletModal = () => setIsWalletModalOpen(true);
  const closeWalletModal = () => setIsWalletModalOpen(false);

  /**
   * Why we need modal root here:
   * useClickOutside closes the menu when clicked "outside the menu".
   * The modal is mounted in a portal, so it's "outside the menu", causing the menu to close when modal clicked.
   * After the menu is closed, the modal disappears as well because the <EzSignlessTransactions /> component,
   * which encapsulates the portal modal component, is unmounted from the menu.
   */
  const modalRootRef = useRootModalRef();

  useClickOutside(
    () => {
      closeMenu();
    },
    menuRef,
    modalRootRef,
  );

  return (
    <>
      <div className={clsx(styles.container, className?.container)} ref={menuRef}>
        <Wallet accountButtonClassName={styles.accountButton} />

        {account && (
          <>
            <div className={styles.contextMenuWrapper}>
              <Button
                color="transparent"
                icon={
                  isMenuOpen
                    ? () => <CrossSVG className={styles.burger} />
                    : () => <BurgerMenuSVG className={styles.burger} />
                }
                className={clsx(className?.icon)}
                onClick={isMenuOpen ? closeMenu : openMenu}
              />
              {isMenuOpen && (
                <motion.div
                  className={clsx(styles.dropdownContainer, className?.dropdown)}
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}>
                  <div className={styles.dropdownHeader}>
                    <Button color="transparent" icon={CrossSVG} className={styles.closeIcon} onClick={closeMenu} />
                  </div>
                  <MenuOptions className={className?.menuOptions} customItems={customItems} onClose={closeMenu} />
                </motion.div>
              )}
            </div>

            {isMenuOpen && (
              <div className={clsx(styles.mobileWrapper, className?.mobileMenuWrapper)}>
                <MobileMenu
                  className={className?.mobileMenu}
                  onClose={closeMenu}
                  onChangeAccountClick={openWalletModal}>
                  <MenuOptions customItems={customItems} className={className?.menuOptions} onClose={closeMenu} />
                </MobileMenu>
              </div>
            )}
          </>
        )}
      </div>

      {isWalletModalOpen && <WalletModal close={closeWalletModal} />}
    </>
  );
}
