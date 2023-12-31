import { createContext, Dispatch, ReactNode, SetStateAction, useContext, useState } from 'react';

type Program = {
  isPending: boolean;
  setIsPending: Dispatch<SetStateAction<boolean>>;
};

export const AppCtx = createContext({} as Program);

const useProgram = (): Program => {
  const [isPending, setIsPending] = useState<boolean>(false);

  return {
    isPending,
    setIsPending,
  };
};

export const useApp = () => useContext(AppCtx);

export function AppProvider({ children }: { children: ReactNode }) {
  const { Provider } = AppCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
