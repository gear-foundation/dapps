import Identicon from '@polkadot/react-identicon';
import { decodeAddress } from '@gear-js/api';
import clsx from 'clsx';

import { ReactComponent as CopySVG } from '../../assets/copy.svg';
import { ReactComponent as ExitSVG } from '../../assets/exit.svg';
import { WALLETS } from '../../consts';
import { useWallet } from '../../hooks';
import { WalletItem } from '../wallet-item';
import styles from './wallet-modal.module.css';
import { useAccount } from '@gear-js/react-hooks';
import { Modal } from '@gear-js/ui';

type Props = {
  close: () => void;
};

function WalletModal({ close }: Props) {
  const { extensions, account, login, logout } = useAccount();

  const { wallet, walletAccounts, setWalletId, resetWalletId, getWalletAccounts, saveWallet, removeWallet } =
    useWallet();

  const getWallets = () =>
    WALLETS.map(([id, { SVG, name }]) => {
      const isEnabled = extensions?.some((extension) => extension.name === id);
      const status = isEnabled ? 'Enabled' : 'Disabled';

      const accountsCount = getWalletAccounts(id)?.length;
      const accountsStatus = `${accountsCount} ${accountsCount === 1 ? 'account' : 'accounts'}`;

      const onClick = () => setWalletId(id);

      return (
        <li key={id}>
          <button type="button" className={styles.walletButton} onClick={onClick} disabled={!isEnabled}>
            <WalletItem icon={SVG} name={name} />

            <div className={styles.status}>
              <p className={styles.statusText}>{status}</p>

              {isEnabled && <p className={styles.statusAccounts}>{accountsStatus}</p>}
            </div>
          </button>
        </li>
      );
    });

  const getAccounts = () =>
    walletAccounts?.map((_account) => {
      const { address, meta } = _account;

      const isActive = address === account?.address;
      const buttonClassName = clsx(styles.accountButton, isActive && styles.active);

      const handleClick = () => {
        login(_account);
        saveWallet();
        close();
      };

      const handleCopyClick = () => {
        const decodedAddress = decodeAddress(address);

        navigator.clipboard.writeText(decodedAddress).then(() => console.log('Copied!'));
        close();
      };

      return (
        <li key={address} className={styles.account}>
          <button type="button" className={buttonClassName} onClick={handleClick} disabled={isActive}>
            <Identicon value={address} size={21} theme="polkadot" />
            <span>{meta.name}</span>
          </button>

          <button type="button" onClick={handleCopyClick}>
            <CopySVG />
          </button>
        </li>
      );
    });

  const handleLogoutButtonClick = () => {
    logout();
    removeWallet();
    close();
  };

  return (
    <Modal
      heading="Wallet connection"
      close={close}
      footer={
        wallet ? (
          <div className={styles.footer}>
            <button type="button" className={styles.walletButton} onClick={resetWalletId}>
              <WalletItem icon={wallet.SVG} name={wallet.name} />

              <span className={styles.changeText}>Change</span>
            </button>

            {account && (
              <button type="button" className={styles.logoutButton} onClick={handleLogoutButtonClick}>
                <ExitSVG />
                <span>Logout</span>
              </button>
            )}
          </div>
        ) : null
      }>
      <ul className={styles.list}>{getAccounts() || getWallets()}</ul>
    </Modal>
  );
}

export { WalletModal };
