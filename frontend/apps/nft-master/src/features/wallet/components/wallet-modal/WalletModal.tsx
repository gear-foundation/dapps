import { useAccount } from '@gear-js/react-hooks';
import Identicon from '@polkadot/react-identicon';
import clsx from 'clsx';
import { useNavigate } from 'react-router-dom';

import { copyToClipboard } from '@dapps-frontend/ui';

import { Button, Modal, ScrollArea, Sprite } from '@/components';
import { usePendingUI } from '@/hooks';
import { isMobileDevice } from '@/utils';

import { useAuth } from '../../../auth/hooks';
import { WALLETS } from '../../consts';
import { useWallet } from '../../hooks';
import { WalletItem } from '../wallet-item';

import styles from './WalletModal.module.scss';

type Props = {
  onClose: () => void;
};

export function WalletModal({ onClose }: Props) {
  const navigate = useNavigate();
  const { wallets, isAnyWallet, account, login } = useAccount();
  const accountAddress = account?.address;
  const { setIsPending } = usePendingUI();
  const { signOut } = useAuth();
  const { wallet, walletAccounts, setWalletId, resetWalletId } = useWallet();

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
            <WalletItem icon={SVG} name={name} />

            <div className={styles.status}>
              <p className={styles.statusText}>{isConnected ? 'Enabled' : 'Disabled'}</p>

              {isConnected && <p className={styles.statusAccounts}>{accountsStatus}</p>}
            </div>
          </Button>
        </li>
      );
    });

  const getAccounts = () =>
    walletAccounts?.map((_account) => {
      const { address, meta } = _account;

      const isActive = address === accountAddress;

      const handleAccountClick = () => {
        setIsPending(true);
        login(_account);
        navigate('/');
        onClose();
      };

      const handleCopyClick = () => {
        copyToClipboard({ value: address });
        onClose();
      };

      return (
        <li key={address} className={styles.account}>
          <Button
            variant={isActive ? 'primary' : 'white'}
            className={styles.button}
            onClick={handleAccountClick}
            disabled={isActive}>
            <Identicon value={address} size={20} theme="polkadot" />
            <span>{meta.name}</span>
          </Button>

          <Button variant="text" className={styles.textButton} onClick={handleCopyClick}>
            <Sprite name="copy" size={16} />
          </Button>
        </li>
      );
    });

  const handleLogoutButtonClick = () => {
    signOut();
    navigate('/');
    onClose();
  };

  const isScrollable = (walletAccounts?.length || 0) > 6;

  const render = () => {
    if (!isAnyWallet)
      return isMobileDevice ? (
        <p>
          To use this application on the mobile devices, open this page inside the compatible wallets like SubWallet or
          Nova.
        </p>
      ) : (
        <p>
          A compatible wallet was not found or is disabled. Install it following the{' '}
          <a
            href="https://wiki.vara-network.io/docs/account/create-account/"
            target="_blank"
            rel="noreferrer"
            className={styles.external}>
            instructions
          </a>
          .
        </p>
      );

    if (!walletAccounts)
      return (
        <ScrollArea className={styles.content} type={isScrollable ? 'always' : undefined}>
          <ul className={clsx(styles.list, isScrollable ? styles['list--scroll'] : '')}>{getWallets()}</ul>
        </ScrollArea>
      );

    if (walletAccounts.length)
      return (
        <ScrollArea className={styles.content} type={isScrollable ? 'always' : undefined}>
          <ul className={clsx(styles.list, isScrollable ? styles['list--scroll'] : '')}>{getAccounts()}</ul>
        </ScrollArea>
      );

    return <p>No accounts found. Please open your extension and create a new account or import existing.</p>;
  };

  return (
    <Modal heading="Wallet connection" onClose={onClose}>
      {render()}

      {wallet && (
        <div className={styles.footer}>
          <button type="button" className={styles.walletButton} onClick={resetWalletId}>
            <WalletItem icon={wallet.SVG} name={wallet.name} />

            <Sprite name="edit" width="12" height="13" />
          </button>

          {account && (
            <Button variant="text" className={styles.textButton} onClick={handleLogoutButtonClick}>
              <Sprite name="exit" size={14} />
              <span>Exit</span>
            </Button>
          )}
        </div>
      )}
    </Modal>
  );
}
