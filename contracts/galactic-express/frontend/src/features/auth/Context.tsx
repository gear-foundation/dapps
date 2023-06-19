import { useAccount, useSendMessage as useHooksSendMessage } from '@gear-js/react-hooks';
import { ReactNode, createContext, useContext, useEffect, useState } from 'react';
import { Account } from '@gear-js/react-hooks/dist/esm/types';
import { useGaslessAccount, useGaslessSendMessage, GaslessAccountValue } from '../gasless-account';

type Value = {
  authType: string;
  setAuthType: (value: string) => void;
  account: Account | GaslessAccountValue | undefined;
  useSendMessage: typeof useHooksSendMessage;
  logout: () => void;
};

const AuthContext = createContext({} as Value);
const { Provider } = AuthContext;

function AuthProvider({ children }: { children: ReactNode }) {
  const [authType, setAuthType] = useState((localStorage.authType as string | null) || '');

  const { account: walletAccount, logout: hooksLogout } = useAccount();
  const { account: gaslessAccount, logout: gaslessLogout } = useGaslessAccount();

  useEffect(() => {
    if (!authType) return localStorage.removeItem('authType');

    localStorage.setItem('authType', authType);
  }, [authType]);

  const value = {
    authType,
    setAuthType,
    account: authType === 'gasless' ? gaslessAccount : walletAccount,
    useSendMessage: authType === 'gasless' ? useGaslessSendMessage : useHooksSendMessage,
    logout: authType === 'gasless' ? gaslessLogout : hooksLogout,
  };

  return <Provider value={value}>{children}</Provider>;
}

const useAuth = () => useContext(AuthContext);

export { AuthProvider, useAuth };
