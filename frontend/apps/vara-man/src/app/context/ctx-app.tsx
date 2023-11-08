import { createContext, ReactNode, useContext, useState } from 'react';

export const AppCtx = createContext({} as ReturnType<typeof useProgram>);
export const useApp = () => useContext(AppCtx);

function useProgram() {
  const [isPending, setIsPending] = useState<boolean>(false);
  const [isAllowed, setIsAllowed] = useState<boolean>(false);
  const [isSettled, setIsSettled] = useState<boolean>(false);

  return {
    isPending,
    setIsPending,
    isAllowed,
    setIsAllowed,
    isSettled,
    setIsSettled,
  };
}

export function AppProvider({ children }: { children: ReactNode }) {
  const { Provider } = AppCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
