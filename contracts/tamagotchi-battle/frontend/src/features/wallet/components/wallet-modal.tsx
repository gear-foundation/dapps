import { Modal } from '@gear-js/ui';
import { ScrollArea } from 'components/ui/scroll-area';
import { useAccount } from '@gear-js/react-hooks';
import { WalletAccount } from './wallet-account';
import { isLoggedIn } from '../utils';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';

type Props = {
  close: () => void;
};

export function WalletModal({ close }: Props) {
  const { accounts, login } = useAccount();

  const onClick = async (account: InjectedAccountWithMeta) => {
    await login(account);
    close();
  };

  return (
    <Modal heading="Connect" close={close}>
      {accounts ? (
        <ScrollArea className="max-h-80 pr-5 -mr-5" type="auto">
          {accounts.length > 0 ? (
            <ul className="space-y-4 w-full">
              {accounts.map((account) => (
                <li key={account.address}>
                  <WalletAccount
                    address={account.address}
                    name={account.meta.name}
                    isActive={isLoggedIn(account)}
                    onClick={() => onClick(account)}
                  />
                </li>
              ))}
            </ul>
          ) : (
            <p>
              No accounts found. Please open Polkadot extension, create a new account or import existing one and reload
              the page.
            </p>
          )}
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
    </Modal>
  );
}
