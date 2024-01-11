import { AnimatePresence, motion } from 'framer-motion';
import { decodeAddress } from '@gear-js/api';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { Dialog } from '@headlessui/react';
import { copyToClipboard, isMobileDevice } from '@/utils';
import { ReactComponent as CopyIcon } from '../../assets/copy.svg';
import { ReactComponent as EditIcon } from '../../assets/edit-icon.svg';
import { ReactComponent as ExitIcon } from '../../assets/exit.svg';
import { ReactComponent as CrossIcon } from '../../assets/cross-icon.svg';
import { WALLETS } from '../../consts';
import { useWallet } from '../../hooks';
import { WalletItem } from '../wallet-item';
import styles from './wallet-modal.module.css';
import { WalletModalProps } from './wallet-modal.interface';
import { AccountIcon } from '../account-icon';
import { variantsPanel, variantsOverlay } from './wallet-modal.variants';
import { ScrollArea } from '@/components/scroll-area';
import { ArrayElement } from '@/types';
import { Wallets } from '../../types';
import clsx from 'clsx';

function WalletModal({ onClose, open, setOpen }: WalletModalProps) {
  const alert = useAlert();
  const { extensions, account, accounts, login, logout } = useAccount();

  const { wallet, walletAccounts, setWalletId, resetWalletId, getWalletAccounts } = useWallet();

  const sortWallets = (wallets: Wallets): Wallets => {
    const [accountsWallets, subwallet, noAccountsWallets] = wallets.reduce(
      (acc: [Wallets, ArrayElement<Wallets> | null, Wallets], item) => {
        const id = item[0];
        if (id === 'subwallet-js') {
          acc[1] = item;
          return acc;
        }

        if (getWalletAccounts(id)?.length) {
          acc[0].push(item);
          return acc;
        }

        acc[2].push(item);
        return acc;
      },
      [[], null, []],
    );

    const sortedAccountsWallets = accountsWallets.sort(([idA], [idB]) =>
      getWalletAccounts(idA)!.length > getWalletAccounts(idB)!.length ? 1 : -1,
    );

    return subwallet
      ? [
          ...(getWalletAccounts(subwallet[0])?.length
            ? [subwallet, ...sortedAccountsWallets]
            : [...sortedAccountsWallets, subwallet]),
          ...noAccountsWallets,
        ]
      : [...sortedAccountsWallets, ...noAccountsWallets];
  };

  const getWallets = () =>
    sortWallets(WALLETS).map(([id, { SVG, name }]) => {
      const isEnabled = extensions?.some((extension) => extension.name === id);
      const status = isEnabled ? 'Enabled' : 'Disabled';

      const accountsCount = getWalletAccounts(id)?.length || 0;
      const accountsStatus = `${accountsCount} ${accountsCount === 1 ? 'account' : 'accounts'}`;

      const onClick = () => setWalletId(id);

      return (
        <li key={id}>
          <button className={styles.walletButton} onClick={onClick} disabled={!isEnabled}>
            <WalletItem Icon={SVG} name={name} />

            <span className={styles.status}>
              <span className={styles.statusText}>{status}</span>

              {isEnabled && <span className={styles.statusAccounts}>{accountsStatus}</span>}
            </span>
          </button>
        </li>
      );
    });

  const getAccounts = () =>
    walletAccounts?.map((_account) => {
      const { address, meta } = _account;

      const isActive = address === account?.address;

      const handleClick = async () => {
        await login(_account);
        setOpen(false);
        onClose();
      };

      const handleCopyClick = async () => {
        const decodedAddress = decodeAddress(address);
        await copyToClipboard({ value: decodedAddress, alert });
        setOpen(false);
        onClose();
      };

      return (
        <li key={address}>
          <div className={styles.account}>
            <button
              className={clsx(styles.accountButton, isActive ? styles.accountButtonActive : '')}
              onClick={handleClick}
              disabled={isActive}>
              <AccountIcon value={address} className={styles.accountIcon} />
              <span>{meta.name}</span>
            </button>

            <button className={styles.textButton} onClick={handleCopyClick}>
              <CopyIcon className={styles.svgIcon} />
            </button>
          </div>
        </li>
      );
    });

  const handleLogoutButtonClick = () => {
    logout();
    resetWalletId();
    setOpen(false);
    onClose();
  };

  const isScrollable = (walletAccounts?.length || 0) > 6;

  return (
    <AnimatePresence initial={false}>
      {open && (
        <Dialog
          as={motion.div}
          initial="closed"
          animate="open"
          exit="closed"
          static
          className={styles.modal}
          open={open}
          onClose={onClose}>
          <motion.div variants={variantsOverlay} className={styles.backdrop} />

          <div className={styles.wrapper}>
            <div className={styles.container}>
              <Dialog.Panel as={motion.div} variants={variantsPanel} className={styles.modalContent}>
                <div className={styles.header}>
                  <Dialog.Title as="h2" className={styles.title}>
                    Wallet connection
                  </Dialog.Title>
                  <button className={styles.close} onClick={onClose}>
                    <CrossIcon />
                  </button>
                </div>
                {accounts?.length ? (
                  <ScrollArea className={styles.content} type={isScrollable ? 'always' : undefined}>
                    <ul className={clsx(styles.list, isScrollable ? styles['list--scroll'] : '')}>
                      {getAccounts() || getWallets()}
                    </ul>
                  </ScrollArea>
                ) : (
                  <>
                    {isMobileDevice ? (
                      <p>
                        To use this application on the mobile devices, open this page inside the compatible wallets like
                        SubWallet or Nova.
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
                      <WalletItem Icon={wallet.SVG} name={wallet.name} />

                      <EditIcon />
                    </button>

                    {account && (
                      <button className={styles.textButton} onClick={handleLogoutButtonClick}>
                        <ExitIcon className={styles.svgIcon} />
                        <span>Exit</span>
                      </button>
                    )}
                  </div>
                )}
              </Dialog.Panel>
            </div>
          </div>
        </Dialog>
      )}
    </AnimatePresence>
  );
}

export { WalletModal };
