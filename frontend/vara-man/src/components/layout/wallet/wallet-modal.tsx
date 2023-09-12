import type { InjectedAccountWithMeta } from '@polkadot/extension-inject/types'
import { useAccount } from '@gear-js/react-hooks'
import { buttonStyles } from '@gear-js/ui';
import clsx from 'clsx';

import { useWallet } from '@/app/hooks/use-wallet'

import { ScrollArea } from '@/components/ui/scroll-area'
import {
  PopupContainer,
  PopupContainerProps,
} from '@/components/common/popup-container'

import { Button } from '@/components/ui/button'

import { WALLETS } from './const';
import { WalletItem } from './wallet-item';

import ExitSVG from '@/assets/images/wallet/exit.svg';

import styles from './WalletModal.module.scss';

type Props = Pick<PopupContainerProps, 'isOpen' | 'setIsOpen'> & {
  accounts: InjectedAccountWithMeta[] | undefined
}

export const WalletModal = ({ accounts, isOpen, setIsOpen }: Props) => {
  const { extensions, account, login, logout } = useAccount();
  const { wallet, walletAccounts, setWalletId, resetWalletId, getWalletAccounts, saveWallet, removeWallet } =
    useWallet();

  const accountAddress = account?.address;

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
            <span className={styles.wallet}>
              <img src={SVG} alt="" className={styles.icon} />
              {name}
            </span>

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

      const handleAccountClick = () => {
        login(_account);
        saveWallet();
        setIsOpen((_) => !_)
      };

      return (
        <li key={_account.address} className="flex items-center gap-2">
          <WalletItem list={_account} onChange={handleAccountClick} />
        </li>
      )
    });

  const handleLogoutButtonClick = () => {
    logout();
    removeWallet();
    setIsOpen((_) => !_)
  };

  return (
    <PopupContainer title="Connect" setIsOpen={setIsOpen} isOpen={isOpen}>
      {accounts ? (
        <ScrollArea className={styles.content} type="auto">
          <ul className={styles.list}>{getAccounts() || getWallets()}</ul>
        </ScrollArea>
      ) : (
        <p>
          Polkadot extension was not found or disabled. Please,{' '}
          <a
            href="https://polkadot.js.org/extension/"
            target="_blank"
            rel="noreferrer"
          >
            install it
          </a>
          .
        </p>
      )}

      {wallet && (
        <footer className={styles.footer}>
          <button type="button" className={styles.walletButton} onClick={resetWalletId}>
            <img src={wallet.SVG} alt={wallet.name} />
            {wallet.name}
          </button>

          {accountAddress && (
            <Button variant="text" className={styles.walletButton} onClick={handleLogoutButtonClick}>
              <img src={ExitSVG} alt="" />
              Exit
            </Button>
          )}
        </footer>
      )}
    </PopupContainer>
  )
}
