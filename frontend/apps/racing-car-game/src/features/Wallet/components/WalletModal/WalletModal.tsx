import { useNavigate } from 'react-router-dom';
import { AnimatePresence, motion } from 'framer-motion';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { Dialog } from '@headlessui/react';
import { cx, copyToClipboard, isMobileDevice } from '@/utils';
import { ReactComponent as CopyIcon } from '@/assets/icons/binary-code.svg';
import { ReactComponent as EditIcon } from '@/assets/icons/pen-edit-icon.svg';
import { ReactComponent as ExitIcon } from '@/assets/icons/exit-icon.svg';
import { ReactComponent as CrossIcon } from '@/assets/icons/cross-icon.svg';
import { WALLETS } from '../../consts';
import { useWallet } from '../../hooks';
import { WalletItem } from '../WalletItem';
import styles from './WalletModal.module.scss';
import { WalletModalProps } from './WalletModal.interface';
import { useAuth } from '@/features/Auth/hooks';
import { PLAY } from '@/App.routes';
import { AccountIcon } from '../account-icon';
import { variantsPanel, variantsOverlay } from '@/components/Modal/modal.variants';
import { ScrollArea } from '@/components/ScrollArea';

function WalletModal({ onClose, open, setOpen }: WalletModalProps) {
  const alert = useAlert();
  const { wallets, isAnyWallet, account } = useAccount();
  const { signIn, signOut } = useAuth();
  const navigate = useNavigate();

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
            className={styles.walletButton}
            onClick={() => (isConnected ? setWalletId(id) : connect?.())}
            disabled={!isEnabled}>
            <WalletItem icon={SVG} name={name} />

            <span className={styles.status}>
              <span className={styles.statusText}>{isConnected ? 'Enabled' : 'Disabled'}</span>

              {isConnected && <span className={styles.statusAccounts}>{accountsStatus}</span>}
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
        await signIn(_account);
        navigate(PLAY);
        setOpen(false);
        onClose();
      };

      const handleCopyClick = async () => {
        await copyToClipboard({ value: address, alert });
        setOpen(false);
        onClose();
      };

      return (
        <li key={address}>
          <div className={styles.account}>
            <button
              className={cx(styles.accountButton, isActive ? styles.accountButtonActive : '')}
              onClick={handleClick}
              disabled={isActive}>
              <AccountIcon value={address} className={styles.accountIcon} />
              <span>{meta.name}</span>
            </button>

            <button className={styles.textButton} onClick={handleCopyClick}>
              <CopyIcon />
            </button>
          </div>
        </li>
      );
    });

  const handleLogoutButtonClick = () => {
    signOut();
    setOpen(false);
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
          <ul className={cx(styles.list, isScrollable ? styles['list--scroll'] : '')}>{getWallets()}</ul>
        </ScrollArea>
      );

    if (walletAccounts.length)
      return (
        <ScrollArea className={styles.content} type={isScrollable ? 'always' : undefined}>
          <ul className={cx(styles.list, isScrollable ? styles['list--scroll'] : '')}>{getAccounts()}</ul>
        </ScrollArea>
      );

    return <p>No accounts found. Please open your extension and create a new account or import existing.</p>;
  };

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
          onClose={setOpen}>
          <motion.div variants={variantsOverlay} className={styles.modal__backdrop} />

          <div className={styles.modal__wrapper}>
            <div className={styles.modal__container}>
              <Dialog.Panel as={motion.div} variants={variantsPanel} className={styles.modal__content}>
                <div className={styles.modal__header}>
                  <Dialog.Title as="h2" className={styles.modal__title}>
                    Wallet connection
                  </Dialog.Title>
                  <button className={styles.modal__close} onClick={() => setOpen(false)}>
                    <CrossIcon />
                  </button>
                </div>
                {render()}

                {wallet && (
                  <div className={styles.footer}>
                    <button type="button" className={styles.walletButton} onClick={resetWalletId}>
                      <WalletItem icon={wallet.SVG} name={wallet.name} />

                      <EditIcon />
                    </button>

                    {account && (
                      <button className={styles.textButton} onClick={handleLogoutButtonClick}>
                        <ExitIcon />
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
