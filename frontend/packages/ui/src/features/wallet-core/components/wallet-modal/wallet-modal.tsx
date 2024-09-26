import { useAccount, useAlert } from '@gear-js/react-hooks';
import { buttonStyles, Button as GearButton, Modal as GearModal } from '@gear-js/ui';
import { Button as VaraButton, Modal as VaraModal } from '@gear-js/vara-ui';
import cx from 'clsx';
import { ReactNode } from 'react';

import { copyToClipboard, isMobileDevice } from '@/utils';

import { ReactComponent as CopySVG } from '../../assets/copy.svg';
import { ReactComponent as EditSVG } from '../../assets/edit-icon.svg';
import { ReactComponent as ExitSVG } from '../../assets/exit.svg';
import { WALLETS } from '../../consts';
import { useWallet } from '../../hooks';
import { VaraAccountButton, GearAccountButton } from '../account-button';
import { WalletItem } from '../wallet-item';
import styles from './wallet-modal.module.css';

type Props = {
  variant?: 'gear' | 'vara';
  close: () => void;
};

function VaraWalletButton(props: { children: ReactNode; disabled: boolean; onClick: () => void }) {
  return <VaraButton className={styles.walletButton} color="light" size="small" block {...props} />;
}

function GearWalletButton(props: { children: ReactNode; disabled: boolean; onClick: () => void }) {
  return (
    <button
      type="button"
      className={cx(
        buttonStyles.button,
        buttonStyles.large,
        buttonStyles.light,
        buttonStyles.block,
        styles.walletButton,
      )}
      {...props}
    />
  );
}

const WALLET_BUTTON = {
  gear: GearWalletButton,
  vara: VaraWalletButton,
} as const;

const ACCOUNT_BUTTON = {
  gear: (props: { address: string; name: string | undefined; color: 'primary' | 'light'; onClick: () => void }) => (
    <GearAccountButton size="large" block {...props} />
  ),
  vara: (props: { address: string; name: string | undefined; color: 'primary' | 'light'; onClick: () => void }) => (
    <VaraAccountButton size="small" block {...props} />
  ),
} as const;

const BUTTON = {
  gear: GearButton,
  vara: VaraButton,
} as const;

const CHANGE_WALLET_BUTTON = {
  gear: (props: { children: ReactNode; onClick: () => void }) => (
    <button type="button" className={cx(buttonStyles.button, buttonStyles.transparent)} {...props} />
  ),
  vara: (props: { children: ReactNode; onClick: () => void }) => <VaraButton color="transparent" {...props} />,
} as const;

const MODAL = {
  gear: GearModal,
  vara: VaraModal,
} as const;

function WalletModal({ variant = 'vara', close }: Props) {
  const alert = useAlert();
  const { wallets, isAnyWallet, account, login, logout } = useAccount();
  const { wallet, walletAccounts, setWalletId, resetWalletId } = useWallet();

  const variantCn = styles[variant];
  const WalletButton = WALLET_BUTTON[variant];
  const AccountButton = ACCOUNT_BUTTON[variant];
  const Button = BUTTON[variant];
  const ChangeWalletButton = CHANGE_WALLET_BUTTON[variant];
  const Modal = MODAL[variant];

  const getWallets = () =>
    WALLETS.map(([id, { SVG, name }]) => {
      const { status, accounts, connect } = wallets?.[id] || {};
      const isEnabled = Boolean(status);
      const isConnected = status === 'connected';

      const accountsCount = accounts?.length || 0;
      const accountsStatus = `${accountsCount} ${accountsCount === 1 ? 'account' : 'accounts'}`;

      return (
        <li key={id}>
          <WalletButton onClick={() => (isConnected ? setWalletId(id) : connect?.())} disabled={!isEnabled}>
            <WalletItem SVG={SVG} name={name} />

            <span className={styles.status}>
              <span className={cx(styles.statusText, variantCn)}>{isConnected ? 'Enabled' : 'Disabled'}</span>

              {isConnected && <span className={cx(styles.statusAccounts, variantCn)}>{accountsStatus}</span>}
            </span>
          </WalletButton>
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
        copyToClipboard({ value: address, alert });
        close();
      };

      return (
        <li key={address} className={styles.account}>
          <AccountButton address={address} name={meta.name} color={color} onClick={handleClick} />

          <Button
            icon={CopySVG}
            color="transparent"
            onClick={handleCopyClick}
            className={cx(styles.copyButton, variantCn)}
          />
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
      return isMobileDevice ? (
        <p className={cx(styles.text, variantCn)}>
          To use this application on mobile devices, open this page inside compatible wallets like Nova or SubWallet.
        </p>
      ) : (
        <p className={cx(styles.text, variantCn)}>
          A compatible wallet was not found or is disabled. Install it following the{' '}
          <a href="https://wiki.vara-network.io/docs/account/create-account/" target="_blank" rel="noreferrer">
            instructions
          </a>
          .
        </p>
      );

    if (!walletAccounts) return <ul className={styles.list}>{getWallets()}</ul>;
    if (walletAccounts.length) return <ul className={styles.list}>{getAccounts()}</ul>;

    return (
      <p className={cx(styles.text, variantCn)}>
        No accounts found. Please open your extension and create a new account or import existing.
      </p>
    );
  };

  const renderFooter = () => {
    if (!wallet) return null;

    return (
      <div className={styles.footer}>
        <ChangeWalletButton onClick={resetWalletId}>
          <WalletItem SVG={wallet.SVG} name={wallet.name} />
          <EditSVG className={cx(styles.editIcon, variantCn)} />
        </ChangeWalletButton>

        {account && (
          <Button
            icon={ExitSVG}
            text="Logout"
            color="transparent"
            onClick={handleLogoutButtonClick}
            className={cx(styles.logoutButton, variantCn)}
          />
        )}
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
