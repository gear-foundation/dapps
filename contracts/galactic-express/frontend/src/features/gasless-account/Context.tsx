import { decodeAddress } from '@gear-js/api';
import { useApi } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { ReactNode, createContext, useContext, useEffect, useState } from 'react';
import { Account } from '@gear-js/react-hooks/dist/esm/types';

type GaslessAccount = {
  publicKey: string | null;
  privateKey: string | null;
};

type GaslessAccountValue = GaslessAccount & {
  decodedAddress: HexString | undefined;
  balance: Account['balance'] | undefined;
};

type Value = {
  account: GaslessAccountValue;
  setAccount: (value: GaslessAccount) => void;
  isLoggedIn: boolean;
  logout: () => void;
};

const AccountContext = createContext({} as Value);

function GaslessAccountProvider({ children }: { children: ReactNode }) {
  const { api } = useApi();

  const [account, setAccount] = useState<GaslessAccount>({
    publicKey: sessionStorage.templatePublicKey,
    privateKey: sessionStorage.templatePrivateKey,
  });

  const decodedAddress = account.publicKey ? decodeAddress(account.publicKey) : undefined;

  const [balance, setBalance] = useState<Account['balance']>();

  const isLoggedIn = !!(account.publicKey && account.privateKey);

  const logout = () => {
    setAccount({ publicKey: null, privateKey: null });
  };

  useEffect(() => {
    if (!isLoggedIn) {
      sessionStorage.removeItem('templatePublicKey');
      sessionStorage.removeItem('templatePrivateKey');
      return;
    }

    sessionStorage.setItem('templatePublicKey', account.publicKey as string);
    sessionStorage.setItem('templatePrivateKey', account.privateKey as string);

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isLoggedIn]);

  useEffect(() => {
    if (!api || !decodedAddress) return setBalance(undefined);

    api.balance
      .findOut(decodedAddress)
      .then((result) => result.toHuman().split(' '))
      .then(([value, unit]) => setBalance({ value, unit }));
  }, [api, decodedAddress]);

  return (
    <AccountContext.Provider
      // eslint-disable-next-line react/jsx-no-constructed-context-values
      value={{ account: { ...account, decodedAddress, balance }, setAccount, isLoggedIn, logout }}>
      {children}
    </AccountContext.Provider>
  );
}

const useGaslessAccount = () => useContext(AccountContext);

export { GaslessAccountProvider, useGaslessAccount };
export type { GaslessAccountValue };
