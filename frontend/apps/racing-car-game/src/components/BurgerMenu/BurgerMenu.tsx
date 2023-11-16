import { useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { cx } from '@/utils';
import styles from './BurgerMenu.module.scss';
import { BurgerMenuProps } from './BurgerMenu.interfaces';
import closeMenuIcon from '@/assets/icons/cross-icon.svg';
import { Button } from '@/ui';
import { WalletModal } from '@/features/Wallet/components';
import { WalletInfo } from '@/features/Wallet/components/WalletInfo';

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
