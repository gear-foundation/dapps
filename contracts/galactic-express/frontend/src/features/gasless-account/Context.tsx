import { decodeAddress } from '@gear-js/api';
import { HexString } from '@polkadot/util/types';
import { ReactNode, createContext, useContext, useEffect, useState } from 'react';

type GaslessAccount = {
  publicKey: string | null;
  privateKey: string | null;
};

type GaslessAccountValue = GaslessAccount & { decodedAddress: HexString | undefined };

type Value = {
  account: GaslessAccountValue;
  setAccount: (value: GaslessAccount) => void;
  isLoggedIn: boolean;
  logout: () => void;
};

const AccountContext = createContext({} as Value);

function GaslessAccountProvider({ children }: { children: ReactNode }) {
  const [account, setAccount] = useState<GaslessAccount>({
    publicKey: sessionStorage.templatePublicKey,
    privateKey: sessionStorage.templatePrivateKey,
  });

  const isLoggedIn = !!(account.publicKey && account.privateKey);

  const logout = () => {
    setAccount({ publicKey: null, privateKey: null });
  };

  useEffect(() => {
    if (!isLoggedIn) return;

    sessionStorage.setItem('templatePublicKey', account.publicKey as string);
    sessionStorage.setItem('templatePrivateKey', account.privateKey as string);

    return () => {
      sessionStorage.removeItem('templatePublicKey');
      sessionStorage.removeItem('templatePrivateKey');
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isLoggedIn]);

  const decodedAddress = account.publicKey ? decodeAddress(account.publicKey) : undefined;

  return (
    <AccountContext.Provider
      // eslint-disable-next-line react/jsx-no-constructed-context-values
      value={{ account: { ...account, decodedAddress }, setAccount, isLoggedIn, logout }}>
      {children}
    </AccountContext.Provider>
  );
}

const useGaslessAccount = () => useContext(AccountContext);

export { GaslessAccountProvider, useGaslessAccount };
export type { GaslessAccountValue };
