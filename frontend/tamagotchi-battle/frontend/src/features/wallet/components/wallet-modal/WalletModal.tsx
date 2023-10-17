import { decodeAddress } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { useNavigate } from 'react-router-dom';
import { copyToClipboard, isMobileDevice } from 'app/utils';
import { ScrollArea, SpriteIcon, Button, Modal } from 'components/ui';
import clsx from 'clsx';
import { ArrayElement } from 'app/types';
import { WalletIcon } from '../wallet-icon';
import { WALLETS, Wallets } from '../../consts';
import { useWallet } from '../../hooks';
import { WalletItem } from '../wallet-item';
import styles from './WalletModal.module.scss';

type Props = {
  onClose: () => void;
};

export function WalletModal({ onClose }: Props) {
  const navigate = useNavigate();
  const { extensions, account, accounts, login, logout } = useAccount();
  const accountAddress = account?.address;
  const { wallet, walletAccounts, setWalletId, getWalletAccounts, resetWalletId } = useWallet();

  const sortWallets = (wallets: Wallets): Wallets => {
    const [accountsWallets, subwallet, noAccountsWallets] = wallets.reduce(
      (acc: [Wallets, ArrayElement<Wallets> | null, Wallets], item) => {
        const id = item[0];
        if (id === 'subwallet-js') {
          acc[1] = item;
          return acc;
        }

        if (getWalletAccounts(id).length) {
          acc[0].push(item);
          return acc;
        }

        acc[2].push(item);
        return acc;
      },
      [[], null, []],
    );

    const sortedAccountsWallets = accountsWallets.sort(([idA], [idB]) =>
      getWalletAccounts(idA).length > getWalletAccounts(idB).length ? 1 : -1,
    );

    return subwallet
      ? [
          ...(getWalletAccounts(subwallet[0]).length
            ? [subwallet, ...sortedAccountsWallets]
            : [...sortedAccountsWallets, subwallet]),
          ...noAccountsWallets,
        ]
      : [...sortedAccountsWallets, ...noAccountsWallets];
  };

  const getWallets = () =>
    sortWallets(WALLETS).map(([id, { SVG, name }]) => {
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
        navigate('/');
        onClose();
      };

      const handleCopyClick = async () => {
        await copyToClipboard({ value: decodeAddress(address) });
        onClose();
      };

      return (
        <li key={address} className={styles.account}>
          <Button
            variant={isActive ? 'primary' : 'white'}
            className={styles.button}
            onClick={handleAccountClick}
            disabled={isActive}>
            <WalletIcon address={address} className={styles.accountIcon} />
            <span>{meta.name}</span>
          </Button>

          <Button variant="text" className={styles.textButton} onClick={handleCopyClick}>
            <SpriteIcon name="copy" size={16} color="#fff" />
          </Button>
        </li>
      );
    });

  const handleLogoutButtonClick = () => {
    logout();
    resetWalletId();
    navigate('/');
    onClose();
  };

  const isScrollable = (walletAccounts?.length || 0) > 6;

  return (
    <Modal heading="Wallet connection" onClose={onClose}>
      {accounts.length ? (
        <ScrollArea className={styles.content} type={isScrollable ? 'always' : undefined}>
          <ul className={clsx(styles.list, isScrollable && styles['list--scroll'])}>{getAccounts() || getWallets()}</ul>
        </ScrollArea>
      ) : (
        // eslint-disable-next-line react/jsx-no-useless-fragment
        <>
          {isMobileDevice ? (
            <p>
              To use this application on the mobile devices, open this page inside the compatible wallets like SubWallet
              or Nova.
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
          )}
        </>
      )}

      {wallet && (
        <div className={styles.footer}>
          <button type="button" className={styles.walletButton} onClick={resetWalletId}>
            <WalletItem icon={wallet.SVG} name={wallet.name} />

            <SpriteIcon name="edit" width="12" height="13" />
          </button>

          {account && (
            <Button variant="text" className={styles.textButton} onClick={handleLogoutButtonClick}>
              <SpriteIcon name="exit" size={14} />
              <span>Exit</span>
            </Button>
          )}
        </div>
      )}
    </Modal>
  );
}
