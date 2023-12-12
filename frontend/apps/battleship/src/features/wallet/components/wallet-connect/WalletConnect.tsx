import { Suspense, useEffect } from 'react';
import { decodeAddress } from '@gear-js/api';
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
  const { extensions, account, accounts, login } = useAccount();

  const navigate = useNavigate();
  const location = useLocation();

  const { walletAccounts, setWalletId, getWalletAccounts } = useWallet();

  useEffect(() => {
    const isNovaWallet = window?.walletExtension?.isNovaWallet;

    if (isNovaWallet) {
      setWalletId('polkadot-js');
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const getWallets = () =>
    WALLETS.map(([id, { SVG, name }]) => {
      const isEnabled = extensions?.some((extension) => extension.name === id);
      const status = isEnabled ? 'Enabled' : 'Disabled';

      const accountsCount = getWalletAccounts(id)?.length;
      const accountsStatus = `${accountsCount} ${accountsCount === 1 ? 'account' : 'accounts'}`;

      const onClick = () => setWalletId(id);

      return (
        <li key={id}>
          <Button variant="white" className={styles.walletButton} onClick={onClick} disabled={!isEnabled}>
            <span className={styles.wallet}>
              <SVG className={styles.icon} />
              {name}
            </span>

            <span className={styles.status}>
              <span className={styles.statusText}>{status}</span>

              {isEnabled && <span className={styles.statusAccounts}>{accountsStatus}</span>}
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
        const decodedAddress = decodeAddress(address);
        copyToClipboard({ value: decodedAddress, alert });
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

  return (
    <ModalBottom heading="Connect Wallet" onClose={onClose}>
      <div>
        {accounts?.length ? (
          <ul className={styles.list}>{getAccounts() || getWallets()}</ul>
        ) : (
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
        )}
      </div>
    </ModalBottom>
  );
}
