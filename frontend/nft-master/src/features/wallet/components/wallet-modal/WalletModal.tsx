import { decodeAddress } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { useNavigate } from 'react-router-dom';
import { copyToClipboard } from 'utils';
import { AccountIcon, Button, Modal, ScrollArea } from 'components';
import { ReactComponent as EditSVG } from 'assets/images/icons/edit.svg';
import { ReactComponent as CopySVG } from 'assets/images/icons/copy.svg';
import { ExitSVG } from '../../assets';
import { WALLETS } from '../../consts';
import { useWallet } from '../../hooks';
import { WalletItem } from '../wallet-item';
import styles from './WalletModal.module.scss';

type Props = {
  onClose: () => void;
};

function WalletModal({ onClose }: Props) {
  const navigate = useNavigate();
  const { extensions, account, accounts, login, logout } = useAccount();
  const accountAddress = account?.address;

  const { wallet, walletAccounts, setWalletId, resetWalletId, getWalletAccounts, saveWallet, removeWallet } =
    useWallet();

  const getWallets = () =>
    WALLETS.map(([id, { SVG, name }]) => {
      const isEnabled = extensions.some((extension) => extension.name === id);
      const status = isEnabled ? 'Enabled' : 'Disabled';

      const accountsCount = getWalletAccounts(id).length;
      const accountsStatus = `${accountsCount} ${accountsCount === 1 ? 'account' : 'accounts'}`;

      return (
        <li key={id}>
          <Button variant="white" className={styles.walletButton} onClick={() => setWalletId(id)} disabled={!isEnabled}>
            <WalletItem icon={SVG} name={name} />

            <div className={styles.status}>
              <p className={styles.statusText}>{status}</p>

              {isEnabled && <p className={styles.statusAccounts}>{accountsStatus}</p>}
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
        login(_account);
        saveWallet();
        navigate('/');
        onClose();
      };

      const handleCopyClick = () => {
        const decodedAddress = decodeAddress(address);

        copyToClipboard(decodedAddress);
        onClose();
      };

      return (
        <li key={address} className={styles.account}>
          <Button
            variant={isActive ? 'primary' : 'white'}
            className={styles.button}
            onClick={handleAccountClick}
            disabled={isActive}>
            <AccountIcon value={address} className={styles.accountIcon} />
            <span>{meta.name}</span>
          </Button>

          <Button variant="text" className={styles.textButton} onClick={handleCopyClick}>
            <CopySVG />
          </Button>
        </li>
      );
    });

  const handleLogoutButtonClick = () => {
    logout();
    removeWallet();
    navigate('/');
    onClose();
  };

  return (
    <Modal heading="Wallet connection" onClose={onClose}>
      {accounts.length ? (
        <ScrollArea className={styles.content} type="auto">
          <ul className={styles.list}>{getAccounts() || getWallets()}</ul>
        </ScrollArea>
      ) : (
        <p>
          Polkadot extension was not found or disabled. Please,{' '}
          <a href="https://polkadot.js.org/extension/" target="_blank" rel="noreferrer">
            install it
          </a>
          .
        </p>
      )}

      {wallet && (
        <footer className={styles.footer}>
          <button type="button" className={styles.walletButton} onClick={resetWalletId}>
            <WalletItem icon={wallet.SVG} name={wallet.name} />

            <EditSVG />
          </button>

          {accountAddress && (
            <Button variant="text" className={styles.textButton} onClick={handleLogoutButtonClick}>
              <ExitSVG />
              <span>Exit</span>
            </Button>
          )}
        </footer>
      )}
    </Modal>
  );
}

export { WalletModal };
