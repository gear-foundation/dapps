import { useAccount, useAlert } from '@gear-js/react-hooks';
import { Button, Modal, buttonStyles } from '@gear-js/ui';
import clsx from 'clsx';
import { copyToClipboard } from '@/utils';
import { ReactComponent as CopySVG } from '../../assets/copy.svg';
import { ReactComponent as ExitSVG } from '../../assets/exit.svg';
import { WALLETS } from '../../consts';
import { useWallet } from '../../hooks';
import { AccountButton } from '../account-button';
import { WalletItem } from '../wallet-item';
import styles from './wallet-modal.module.css';

type Props = {
  close: () => void;
};

function WalletModal({ close }: Props) {
  const { wallets, isAnyWallet, account, login, logout } = useAccount();
  const alert = useAlert();
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
          <button
            type="button"
            className={clsx(
              buttonStyles.button,
              buttonStyles.large,
              buttonStyles.light,
              buttonStyles.block,
              styles.walletButton,
              isEnabled && styles.enabled,
            )}
            onClick={() => (isConnected ? setWalletId(id) : connect?.())}
            disabled={!isEnabled}>
            <WalletItem SVG={SVG} name={name} />

            <div className={styles.status}>
              <p className={styles.statusText}>{isConnected ? 'Enabled' : 'Disabled'}</p>

              {isConnected && <p className={styles.statusAccounts}>{accountsStatus}</p>}
            </div>
          </button>
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

      const handleCopyClick = () => {
        copyToClipboard({ alert, value: address });
      };

      return (
        <li key={address} className={styles.account}>
          <AccountButton address={address} name={meta.name} size="large" color={color} onClick={handleClick} block />
          <Button icon={CopySVG} color="transparent" onClick={handleCopyClick} />
        </li>
      );
    });

  const handleLogoutButtonClick = () => {
    logout();
    close();
  };

  return (
    <Modal
      heading="Connect Wallet"
      close={close}
      footer={
        wallet ? (
          <div className={styles.footer}>
            <button
              type="button"
              className={clsx(buttonStyles.button, buttonStyles.transparent)}
              onClick={resetWalletId}>
              <WalletItem SVG={wallet.SVG} name={wallet.name} />

              <span className={styles.changeText}>Change</span>
            </button>

            {account && <Button icon={ExitSVG} text="Logout" color="transparent" onClick={handleLogoutButtonClick} />}
          </div>
        ) : null
      }>
      {isAnyWallet ? (
        <ul className={styles.list}>{getAccounts() || getWallets()}</ul>
      ) : (
        <div className={styles.instruction}>
          <p>A compatible wallet wasn't found or is disabled.</p>
          <p>
            Please, install it following the{' '}
            <a href="https://wiki.vara-network.io/docs/account/create-account/" target="_blank" rel="noreferrer">
              instructions
            </a>
            .
          </p>
        </div>
      )}
    </Modal>
  );
}

export { WalletModal };
