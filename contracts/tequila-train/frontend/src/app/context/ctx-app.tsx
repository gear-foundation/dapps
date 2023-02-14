import { createContext, Dispatch, ReactNode, SetStateAction, useState } from 'react';

type Program = {
  isPending: boolean;
  setIsPending: Dispatch<SetStateAction<boolean>>;
  isAllowed: boolean;
  setIsAllowed: Dispatch<SetStateAction<boolean>>;
  isDataReady: boolean;
  setIsDataReady: Dispatch<SetStateAction<boolean>>;
};

export const AppCtx = createContext({} as Program);

const useProgram = (): Program => {
  const [isPending, setIsPending] = useState<boolean>(false);
  const [isAllowed, setIsAllowed] = useState<boolean>(false);
  const [isDataReady, setIsDataReady] = useState<boolean>(false);

  return {
    isPending,
    setIsPending,
    isAllowed,
    setIsAllowed,
    isDataReady,
    setIsDataReady,
  };
};

export function AppProvider({ children }: { children: ReactNode }) {
  const { Provider } = AppCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
