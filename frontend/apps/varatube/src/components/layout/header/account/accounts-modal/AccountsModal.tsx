import { useAccount } from '@gear-js/react-hooks';
import { isWeb3Injected } from '@polkadot/extension-dapp';
import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';
import { Button, Modal } from '@gear-js/ui';
import { useEffect, useState } from 'react';
import SimpleBar from 'simplebar-react';
import clsx from 'clsx';
import { LOCAL_STORAGE } from 'consts';
import { ReactComponent as LogoutSVG } from 'assets/images/icons/logout.svg';
import { ReactComponent as ArrowSVG } from 'assets/images/icons/arrow.svg';
import { useWallet } from './hooks';
import { Wallets } from './wallets';
import { AccountList } from './account-list';
import styles from './AccountsModal.module.scss';

type Props = {
  close: () => void;
};

function AccountsModal({ close }: Props) {
  const { account, accounts, extensions, login, logout } = useAccount();
  const { wallet, walletId, switchWallet, resetWallet } = useWallet();

  const [isWalletSelection, setIsWalletSelection] = useState(!wallet);

  const toggleWalletSelection = () => setIsWalletSelection((prevValue) => !prevValue);
  const enableWalletSelection = () => setIsWalletSelection(true);
  const disableWalletSelection = () => setIsWalletSelection(false);

  const handleLogoutClick = () => {
    logout();
    close();
  };

  const handleAccountClick = (newAccount: InjectedAccountWithMeta) => {
    if (walletId) {
      login(newAccount);
      localStorage.setItem(LOCAL_STORAGE.WALLET, walletId);
      close();
    }
  };

  useEffect(() => {
    if (walletId) {
      disableWalletSelection();
    } else {
      enableWalletSelection();
    }
  }, [walletId]);

  useEffect(() => {
    const isChosenExtensionExists = extensions?.some((ext) => ext.name === walletId);

    if (!isChosenExtensionExists) resetWallet();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [extensions]);

  const modalClassName = clsx(styles.modal, !isWeb3Injected && styles.empty);
  const heading = isWalletSelection ? 'Choose Wallet' : 'Connect account';

  return (
    <Modal heading={heading} close={close} className={modalClassName}>
      {isWeb3Injected ? (
        <>
          <SimpleBar className={styles.simplebar}>
            {isWalletSelection && (
              <Wallets selectedWalletId={walletId} onWalletClick={switchWallet} extensions={extensions || []} />
            )}

            {!isWalletSelection && (
              <AccountList
                list={accounts!.filter(({ meta }) => meta.source === walletId)}
                address={account?.address}
                toggleAccount={handleAccountClick}
              />
            )}
          </SimpleBar>

          <footer className={styles.footer}>
            {wallet && (
              <Button
                icon={isWalletSelection ? ArrowSVG : wallet.icon}
                text={isWalletSelection ? 'Back' : wallet.name}
                color="transparent"
                onClick={toggleWalletSelection}
                disabled={!wallet}
              />
            )}

            <Button
              icon={LogoutSVG}
              text="Logout"
              color="transparent"
              className={styles.logoutButton}
              onClick={handleLogoutClick}
            />
          </footer>
        </>
      ) : (
        <p>
          Wallet extension was not found or disconnected. Please check how to install a supported wallet and create an
          account{' '}
          <a href="https://wiki.gear-tech.io/docs/idea/account/create-account" target="_blank" rel="noreferrer">
            here
          </a>
          .
        </p>
      )}
    </Modal>
  );
}

export { AccountsModal };
