import Identicon from '@polkadot/react-identicon';
import { decodeAddress } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { Button, Modal, buttonStyles } from '@gear-js/ui';
import clsx from 'clsx';
import { copyToClipboard } from 'utils';
import { ReactComponent as ExitSVG } from '../../assets/images/exit.svg';
import { CopySVG } from '../../assets';
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

      const className = clsx(
        styles.walletButton,
        buttonStyles.button,
        buttonStyles.light,
        buttonStyles.large,
        buttonStyles.block,
      );

      return (
        <li key={id}>
          <button type="button" className={className} onClick={onClick} disabled={!isEnabled}>
            <WalletItem icon={SVG} name={name} />

            <div className={styles.status}>
              <p>{status}</p>
              {isEnabled && <p>{accountsStatus}</p>}
            </div>
          </button>
        </li>
      );
    });

  const getAccounts = () =>
    walletAccounts?.map((_account) => {
      const { address, meta } = _account;
      const isActive = address === account?.address;

      const handleClick = () => {
        login(_account);
        saveWallet();
        onClose();
      };

      const handleCopyClick = () => {
        const decodedAddress = decodeAddress(address);

        copyToClipboard(decodedAddress);
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
    removeWallet();
    onClose();
  };

  const walletButtonClassName = clsx(buttonStyles.button, buttonStyles.transparent);

  return (
    <Modal heading="Wallet connection" close={onClose}>
      <ul className={styles.list}>{getAccounts() || getWallets()}</ul>

      {wallet && (
        <footer className={styles.footer}>
          <button type="button" className={walletButtonClassName} onClick={resetWalletId}>
            <WalletItem icon={wallet.SVG} name={wallet.name} />

            <span className={styles.changeText}>Change</span>
          </button>

          {account && <Button icon={ExitSVG} text="Logout" color="transparent" onClick={handleLogoutButtonClick} />}
        </footer>
      )}
    </Modal>
  );
}

export { WalletModal };
