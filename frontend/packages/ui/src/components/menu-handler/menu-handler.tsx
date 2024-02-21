import { useRef, useState } from 'react';
import { motion } from 'framer-motion';
import { WalletNew as Wallet } from '@/features';
import { Button } from '@gear-js/vara-ui';
import { ReactComponent as BurgerMenuSVG } from './assets/burger-menu.svg';
import { ReactComponent as CrossSVG } from '@/assets/cross-icon.svg';
import styles from './menu-handler.module.css';
import { MenuOptions, MobileMenu } from '..';
import { MenuOptionsClassNameProps } from '../menu-options';
import { MobileMenuClassNameProps } from '../mobile-menu';
import { useClickOutside } from '@/utils';
import clsx from 'clsx';
import { useAccount } from '@gear-js/react-hooks';
import { WalletClassNameProps } from '@/features/wallet-new/components/wallet';

type Props = {
  customItems?: {
    icon?: React.FunctionComponent<React.SVGProps<SVGSVGElement>>;
    option: JSX.Element;
  }[];
  className?: {
    container?: string;
    dropdown?: string;
    mobileMenuWrapper?: string;
    icon?: string;
    menuOptions?: MenuOptionsClassNameProps;
    mobileMenu?: MobileMenuClassNameProps;
    wallet?: WalletClassNameProps;
  };
};

export function MenuHandler({ customItems, className }: Props) {
  const menuRef = useRef<HTMLDivElement>(null);
  const { account } = useAccount();
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const [isWalletModalOpen, setIsWalletModalOpen] = useState(false);

  const openMenu = () => setIsMenuOpen(true);
  const closeMenu = () => setIsMenuOpen(false);

  useClickOutside(() => {
    closeMenu();
  }, menuRef);

  return (
    <div className={clsx(styles.container, className?.container)} ref={menuRef}>
      <div>
        <Wallet
          isWalletModalOpen={isWalletModalOpen}
          walletModalHandler={setIsWalletModalOpen}
          className={className?.wallet}
        />
      </div>

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
                walletModalHandler={setIsWalletModalOpen}>
                <MenuOptions customItems={customItems} className={className?.menuOptions} onClose={closeMenu} />
              </MobileMenu>
            </div>
          )}
        </>
      )}
    </div>
  );
}
