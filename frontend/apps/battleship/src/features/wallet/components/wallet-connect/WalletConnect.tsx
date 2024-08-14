import { Suspense, useEffect } from 'react';
import { useAlert, useAccount } from '@gear-js/react-hooks';
import Identicon from '@polkadot/react-identicon';

import { copyToClipboard } from '@/app/utils';
import { CopyDecoded } from '@/assets/images/';

import { Button } from '@/components/ui/button';
import { ModalBottom } from '@/components/ui/modal';

import { WALLETS } from '../../consts';
import { useWallet } from '../../hooks';

import styles from './WalletConnect.module.scss';
import { useLocation, useNavigate } from 'react-router-dom';
import { ROUTES } from '@/app/consts';

type Props = {
  onClose(): void;
};

export function WalletConnect({ onClose }: Props) {
  const alert = useAlert();
  const { wallets, isAnyWallet, account, login } = useAccount();

  const navigate = useNavigate();
  const location = useLocation();

  const { walletAccounts, setWalletId } = useWallet();

  useEffect(() => {
    const isNovaWallet = window?.walletExtension?.isNovaWallet;

    if (isNovaWallet) {
      setWalletId('polkadot-js');
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const getWallets = () =>
    WALLETS.map(([id, { SVG, name }]) => {
      const { status, accounts, connect } = wallets?.[id] || {};
      const isEnabled = Boolean(status);
      const isConnected = status === 'connected';

      const accountsCount = accounts?.length;
      const accountsStatus = `${accountsCount} ${accountsCount === 1 ? 'account' : 'accounts'}`;

      return (
        <li key={id}>
          <Button
            variant="white"
            className={styles.walletButton}
            onClick={() => (isConnected ? setWalletId(id) : connect?.())}
            disabled={!isEnabled}>
            <span className={styles.wallet}>
              <SVG className={styles.icon} />
              {name}
            </span>

            <span className={styles.status}>
              <span className={styles.statusText}>{isConnected ? 'Enabled' : 'Disabled'}</span>

              {isConnected && <span className={styles.statusAccounts}>{accountsStatus}</span>}
            </span>
          </Button>
        </li>
      );
    });

  const getAccounts = () =>
    walletAccounts?.map((_account) => {
      const { address, meta } = _account;

      const isActive = address === account?.address;

      const handleClick = async () => {
        login(_account);
        onClose();

        const from = location.state?.from?.pathname || ROUTES.HOME;
        navigate(from, { replace: true });
      };

      const handleCopyClick = () => {
        copyToClipboard({ value: address, alert });
        onClose();
      };

      return (
        <li key={address}>
          <div className={styles.account}>
            <Button
              variant={isActive ? 'primary' : 'white'}
              className={styles.accountButton}
              onClick={handleClick}
              disabled={isActive}>
              <Suspense>
                <Identicon value={address} size={20} theme="polkadot" className={styles.accountIcon} />
              </Suspense>
              <span className={styles.name}>{meta.name}</span>
            </Button>

            <Button variant="text" className={styles.textButton} onClick={handleCopyClick}>
              <CopyDecoded />
            </Button>
          </div>
        </li>
      );
    });

  const render = () => {
    if (!isAnyWallet)
      return (
        <p>
          <a href="https://novawallet.io/" target="_blank" rel="noreferrer">
            Nova
          </a>{' '}
          or{' '}
          <a href="https://www.subwallet.app/" target="_blank" rel="noreferrer">
            Subwallet
          </a>{' '}
          extensions was not found or disabled. Please, install it.
        </p>
      );

    if (!walletAccounts) return <ul className={styles.list}>{getWallets()}</ul>;
    if (walletAccounts.length) return <ul className={styles.list}>{getAccounts()}</ul>;

    return <p>No accounts found. Please open your extension and create a new account or import existing.</p>;
  };

  return (
    <ModalBottom heading="Connect Wallet" onClose={onClose}>
      <div>{render()}</div>
    </ModalBottom>
  );
}
