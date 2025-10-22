import { WalletModal } from '@gear-js/wallet-connect';
import clsx from 'clsx';
import { EzGaslessTransactions, EzSignlessTransactions, RevokeExpiredVouchers } from 'gear-ez-transactions';
import { useState } from 'react';

import { MenuOptions, MobileMenu } from '@dapps-frontend/ui';

import { SIGNLESS_ALLOWED_ACTIONS } from '@/app/consts';

import { AccountInfo } from './account-info';
import styles from './header.module.scss';
import { Logo } from './logo';

type HeaderProps = BaseComponentProps;

export function Header({ children }: HeaderProps) {
  const [isOpenMenu, setIsOpenMenu] = useState(false);
  const openAndCloseMenu = () => setIsOpenMenu(!isOpenMenu);
  const closeMenu = () => setIsOpenMenu(false);

  const [isWalletModalOpen, setIsWalletModalOpen] = useState(false);
  const openWalletModal = () => setIsWalletModalOpen(true);
  const closeWalletModal = () => setIsWalletModalOpen(false);

  return (
    <>
      <header className={clsx(styles.header, isOpenMenu && styles.open)}>
        <div className={styles.header__container}>
          {isOpenMenu ? <Logo className={styles.header__logo} /> : children}
          <AccountInfo openWallet={openAndCloseMenu} isOpen={isOpenMenu} className={styles.accountInfo} />
        </div>

        {isOpenMenu && (
          <div className={clsx(styles.mobileWrapper)}>
            <MobileMenu onClose={closeMenu} onChangeAccountClick={openWalletModal}>
              <MenuOptions
                customItems={[
                  { key: 'signless', option: <EzSignlessTransactions allowedActions={SIGNLESS_ALLOWED_ACTIONS} /> },
                  { key: 'gasless', option: <EzGaslessTransactions /> },
                  { key: 'revoke-vouchers', option: <RevokeExpiredVouchers /> },
                ]}
                onClose={closeMenu}
              />
            </MobileMenu>
          </div>
        )}
      </header>
      {isWalletModalOpen && <WalletModal close={closeWalletModal} />}
    </>
  );
}
