import Identicon from '@polkadot/react-identicon';
import { decodeAddress } from '@gear-js/api';
import clsx from 'clsx';
import { copyToClipBoard } from 'utils';
import { Modal } from 'components';
import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/ui';
import { CopySVG, ExitSVG } from '../../assets';
import { ReactComponent as EditSVG } from '../../assets/images/edit.svg';
import { WALLETS } from '../../consts';
import { useWallet } from '../../hooks';
import { WalletItem } from '../wallet-item';
import styles from './WalletModal.module.scss';

type Props = {
  onClose: () => void;
};

function WalletModal({ onClose }: Props) {
  const { extensions, account, login, logout } = useAccount();

  const { wallet, walletAccounts, setWalletId, resetWalletId, getWalletAccounts, saveWallet, removeWallet } =
    useWallet();

  const getWallets = () =>
    WALLETS.map(([id, { SVG, name }]) => {
      const isEnabled = extensions.some((extension) => extension.name === id);
      const status = isEnabled ? 'Enabled' : 'Disabled';

      const accountsCount = getWalletAccounts(id).length;
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
        onClose();
      };

      const handleCopyClick = () => {
        const decodedAddress = decodeAddress(address);

        copyToClipBoard(decodedAddress);
        onClose();
      };

      return (
        <li key={address} className={styles.account}>
          <button type="button" className={buttonClassName} onClick={handleClick} disabled={isActive}>
            <Identicon value={address} size={21} theme="polkadot" />
            <span>{meta.name}</span>
          </button>

          <Button icon={CopySVG} color="transparent" onClick={handleCopyClick} className={styles.accountCopyButton} />
        </li>
      );
    });

  const handleLogoutButtonClick = () => {
    logout();
    removeWallet();
    onClose();
  };

  return (
    <Modal heading="Wallet connection" onClose={onClose}>
      <ul className={styles.list}>{getAccounts() || getWallets()}</ul>

      {wallet && (
        <footer className={styles.footer}>
          <button type="button" className={styles.walletButton} onClick={resetWalletId}>
            <WalletItem icon={wallet.SVG} name={wallet.name} />

            <Button icon={EditSVG} color="transparent" onClick={resetWalletId} />
          </button>

          {account && (
            <Button
              icon={ExitSVG}
              text="Logout"
              color="transparent"
              className={styles.logoutButton}
              onClick={handleLogoutButtonClick}
            />
          )}
        </footer>
      )}
    </Modal>
  );
}

export { WalletModal };
