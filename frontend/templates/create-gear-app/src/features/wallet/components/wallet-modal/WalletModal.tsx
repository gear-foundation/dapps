import { decodeAddress } from '@gear-js/api';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { Button, Modal, buttonStyles } from '@gear-js/ui';
import { Identicon } from '@polkadot/react-identicon';
import clsx from 'clsx';

import { copyToClipboard } from '@/utils';

import { CopySVG, ExitSVG } from '../../assets';
import { WALLETS } from '../../consts';
import { useWallet } from '../../hooks';
import { WalletItem } from '../wallet-item';

import styles from './WalletModal.module.scss';

type Props = {
  onClose: () => void;
};

function WalletModal({ onClose }: Props) {
  const { wallets, isAnyWallet, account, login, logout } = useAccount();
  const alert = useAlert();

  const { wallet, walletAccounts, setWalletId, resetWalletId } = useWallet();

  const renderWallets = () =>
    WALLETS.map(([id, { SVG, name }]) => {
      const { status, accounts, connect } = wallets?.[id] || {};
      const isEnabled = Boolean(status);
      const isConnected = status === 'connected';

      const accountsCount = accounts?.length;
      const accountsStatus = `${accountsCount} ${accountsCount === 1 ? 'account' : 'accounts'}`;

      return (
        <li key={id}>
          <button
            type="button"
            className={clsx(
              styles.walletButton,
              buttonStyles.button,
              buttonStyles.light,
              buttonStyles.large,
              buttonStyles.block,
            )}
            onClick={() => (isConnected ? setWalletId(id) : connect?.())}
            disabled={!isEnabled}>
            <WalletItem icon={SVG} name={name} />

            <div className={styles.status}>
              <p>{isConnected ? 'Enabled' : 'Disabled'}</p>
              {isConnected && <p>{accountsStatus}</p>}
            </div>
          </button>
        </li>
      );
    });

  const renderAccounts = () =>
    walletAccounts?.map((_account) => {
      const { address, meta } = _account;
      const isActive = address === account?.address;

      const handleClick = () => {
        login(_account);
        onClose();
      };

      const handleCopyClick = () => {
        const decodedAddress = decodeAddress(address);

        copyToClipboard(decodedAddress).catch((error) =>
          alert.error(error instanceof Error ? error.message : String(error)),
        );
        onClose();
      };

      const className = clsx(
        styles.accountButton,
        buttonStyles.button,
        buttonStyles.large,
        buttonStyles.block,
        isActive ? buttonStyles.primary : buttonStyles.light,
      );

      return (
        <li key={address} className={styles.account}>
          <button type="button" className={className} onClick={handleClick}>
            <Identicon value={address} size={21} theme="polkadot" />
            <span>{meta.name}</span>
          </button>

          <Button icon={CopySVG} color="transparent" onClick={handleCopyClick} />
        </li>
      );
    });

  const handleLogoutButtonClick = () => {
    logout();
    onClose();
  };

  const walletButtonClassName = clsx(buttonStyles.button, buttonStyles.transparent);

  const renderFooter = () =>
    wallet && (
      <div className={styles.footer}>
        <button type="button" className={walletButtonClassName} onClick={resetWalletId}>
          <WalletItem icon={wallet.SVG} name={wallet.name} />

          <span className={styles.changeText}>Change</span>
        </button>

        {account && <Button icon={ExitSVG} text="Logout" color="transparent" onClick={handleLogoutButtonClick} />}
      </div>
    );

  return (
    <Modal heading="Wallet connection" close={onClose} footer={renderFooter()}>
      {isAnyWallet ? (
        <>
          {!wallet && <ul className={styles.list}>{renderWallets()}</ul>}

          {!!wallet &&
            (walletAccounts?.length ? (
              <ul className={styles.list}>{renderAccounts()}</ul>
            ) : (
              <p>No accounts found. Please open your extension and create a new account or import existing.</p>
            ))}
        </>
      ) : (
        <p>
          Wallet extensions were not found or disconnected. Please check how to install a supported wallet and create an
          account{' '}
          <a
            href="https://wiki.gear-tech.io/docs/idea/account/create-account"
            target="_blank"
            rel="noreferrer"
            className="link-text">
            here
          </a>
          .
        </p>
      )}
    </Modal>
  );
}

export { WalletModal };
