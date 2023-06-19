import { Dispatch, ReactNode, SetStateAction, createContext, useContext, useEffect, useState } from 'react';

type GaslessAccount = {
  publicKey: string | null;
  privateKey: string | null;
};

type Value = {
  account: GaslessAccount;
  setAccount: Dispatch<SetStateAction<GaslessAccount>>;
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

  return (
    <AccountContext.Provider
      // eslint-disable-next-line react/jsx-no-constructed-context-values
      value={{ account, setAccount, isLoggedIn, logout }}>
      {children}
    </AccountContext.Provider>
  );
}

const useGaslessAccount = () => useContext(AccountContext);

export { GaslessAccountProvider, useGaslessAccount };
