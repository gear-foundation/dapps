import { ReactNode, createContext, useContext, useEffect, useState } from 'react';

type Value = {
  authType: string;
  setAuthType: (value: string) => void;
};

const AuthContext = createContext({} as Value);
const { Provider } = AuthContext;

function AuthProvider({ children }: { children: ReactNode }) {
  const [authType, setAuthType] = useState((localStorage.authType as string | null) || '');

  useEffect(() => {
    if (!authType) return localStorage.removeItem('authType');

    localStorage.setItem('authType', authType);
  }, [authType]);

  return <Provider value={{ authType, setAuthType }}>{children}</Provider>;
}

const useAuth = () => useContext(AuthContext);

export { AuthProvider, useAuth };
