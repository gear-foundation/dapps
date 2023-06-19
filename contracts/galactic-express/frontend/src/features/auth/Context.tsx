import { useAccount, useSendMessage as useHooksSendMessage } from '@gear-js/react-hooks';
import { ReactNode, createContext, useContext, useEffect, useState } from 'react';
import { useGaslessAccount, useGaslessSendMessage } from '../gasless-account';

type Value = {
  authType: string;
  setAuthType: (value: string) => void;
  accountAddress: string | null | undefined;
  useSendMessage: typeof useHooksSendMessage;
};

const AuthContext = createContext({} as Value);
const { Provider } = AuthContext;

function AuthProvider({ children }: { children: ReactNode }) {
  const [authType, setAuthType] = useState((localStorage.authType as string | null) || '');

  const { account: walletAccount } = useAccount();
  const { account: gaslessAccount } = useGaslessAccount();

  useEffect(() => {
    if (!authType) return localStorage.removeItem('authType');

    localStorage.setItem('authType', authType);
  }, [authType]);

  const value = {
    authType,
    setAuthType,
    accountAddress: authType === 'gasless' ? gaslessAccount.publicKey : walletAccount?.decodedAddress,
    useSendMessage: authType === 'gasless' ? useGaslessSendMessage : useHooksSendMessage,
  };

  return <Provider value={value}>{children}</Provider>;
}

const useAuth = () => useContext(AuthContext);

export { AuthProvider, useAuth };
