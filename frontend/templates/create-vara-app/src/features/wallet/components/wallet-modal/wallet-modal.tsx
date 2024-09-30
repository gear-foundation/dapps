import { useAccount } from '@gear-js/react-hooks';
import { Button, Modal } from '@gear-js/vara-ui';

import { copyToClipboard } from '@/utils';

import CopyIcon from '../../assets/copy.svg?react';
import EditIcon from '../../assets/edit-icon.svg?react';
import ExitSVG from '../../assets/exit.svg?react';
import { IS_MOBILE_DEVICE, WALLETS } from '../../consts';
import { useWallet } from '../../hooks';
import { AccountButton } from '../account-button';
import { WalletItem } from '../wallet-item';

import styles from './wallet-modal.module.css';

type Props = {
  close: () => void;
};

function WalletModal({ close }: Props) {
  const { wallets, isAnyWallet, account, login, logout } = useAccount();
  const { wallet, walletAccounts, setWalletId, resetWalletId } = useWallet();

  const getWallets = () =>
    WALLETS.map(([id, { SVG, name }]) => {
      const { status, accounts, connect } = wallets?.[id] || {};
      const isEnabled = Boolean(status);
      const isConnected = status === 'connected';

      const accountsCount = accounts?.length || 0;
      const accountsStatus = `${accountsCount} ${accountsCount === 1 ? 'account' : 'accounts'}`;

      return (
        <li key={id}>
          <Button
            className={styles.walletButton}
            color="light"
            size="small"
            onClick={() => (isConnected ? setWalletId(id) : connect?.())}
            disabled={!isEnabled}
            block>
            <WalletItem SVG={SVG} name={name} />

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
      const color = isActive ? 'primary' : 'light';

      const handleClick = () => {
        if (isActive) return;

        login(_account);
        close();
      };

      const handleCopyClick = async () => {
        await copyToClipboard(address);
        close();
      };

      return (
        <li key={address} className={styles.account}>
          <AccountButton size="small" address={address} name={meta.name} color={color} onClick={handleClick} block />
          <Button icon={CopyIcon} color="transparent" onClick={handleCopyClick} />
        </li>
      );
    });

  const handleLogoutButtonClick = () => {
    logout();
    resetWalletId();
    close();
  };

  const render = () => {
    if (!isAnyWallet)
      return IS_MOBILE_DEVICE ? (
        <p>
          To use this application on the mobile devices, open this page inside the compatible wallets like SubWallet or
          Nova.
        </p>
      ) : (
        <p>
          A compatible wallet was not found or is disabled. Install it following the{' '}
          <a href="https://wiki.vara-network.io/docs/account/create-account/" target="_blank" rel="noreferrer">
            instructions
          </a>
          .
        </p>
      );

    if (!walletAccounts) return <ul className={styles.list}>{getWallets()}</ul>;
    if (walletAccounts.length) return <ul className={styles.list}>{getAccounts()}</ul>;

    return <p>No accounts found. Please open your extension and create a new account or import existing.</p>;
  };

  const renderFooter = () => {
    if (!wallet) return null;

    return (
      <div className={styles.footer}>
        <Button color="transparent" onClick={resetWalletId}>
          <WalletItem SVG={wallet.SVG} name={wallet.name} />
          <EditIcon />
        </Button>

        {account && <Button icon={ExitSVG} text="Logout" color="transparent" onClick={handleLogoutButtonClick} />}
      </div>
    );
  };

  return (
    <Modal heading="Connect Wallet" close={close} footer={renderFooter()}>
      {render()}
    </Modal>
  );
}

export { WalletModal };
