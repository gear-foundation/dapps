import { useAccount } from '@gear-js/react-hooks';
import { useState } from 'react';

import closeMenuIcon from '@/assets/icons/cross-icon.svg';
import { WalletModal } from '@/features/Wallet/components';
import { WalletInfo } from '@/features/Wallet/components/WalletInfo';
import { Button } from '@/ui';
import { cx } from '@/utils';

import { BurgerMenuProps } from './BurgerMenu.interfaces';
import styles from './BurgerMenu.module.scss';

// @deprecated
function BurgerMenu({ burgerMenuHandler }: BurgerMenuProps) {
  const { account } = useAccount();
  const [isWalletModalOpen, setIsWalletModalOpen] = useState<boolean>(false);

  const handleCloseWalletModal = () => {
    setIsWalletModalOpen(false);
  };

  return (
    <div className={cx(styles['burger-menu'])}>
      <div className={cx(styles['burger-menu-header'])}>
        <div className={cx(styles['burger-menu-close-icon'])}>
          <Button variant="icon" label="" icon={closeMenuIcon} onClick={burgerMenuHandler} />
        </div>
        <WalletInfo account={account} withoutBalance />
      </div>

      <div className={cx(styles['burger-menu-body'])} />
      {isWalletModalOpen && (
        <WalletModal open={isWalletModalOpen} setOpen={setIsWalletModalOpen} onClose={handleCloseWalletModal} />
      )}
    </div>
  );
}

export { BurgerMenu };
