import { createContext, Dispatch, ReactNode, SetStateAction, useState } from 'react';

type Program = {
  isPending: boolean;
  setIsPending: Dispatch<SetStateAction<boolean>>;
  isAllowed: boolean;
  setIsAllowed: Dispatch<SetStateAction<boolean>>;
  openEmptyPopup: boolean;
  setOpenEmptyPopup: Dispatch<SetStateAction<boolean>>;
  openWinnerPopup: boolean;
  setOpenWinnerPopup: Dispatch<SetStateAction<boolean>>;
};

export const AppCtx = createContext({} as Program);

const useProgram = (): Program => {
  const [isPending, setIsPending] = useState<boolean>(false);
  const [isAllowed, setIsAllowed] = useState<boolean>(false);
  const [openEmptyPopup, setOpenEmptyPopup] = useState<boolean>(false);
  const [openWinnerPopup, setOpenWinnerPopup] = useState<boolean>(false);

  return {
    isPending,
    setIsPending,
    isAllowed,
    setIsAllowed,
    openEmptyPopup,
    setOpenEmptyPopup,
    openWinnerPopup,
    setOpenWinnerPopup,
  };
};

export function AppProvider({ children }: { children: ReactNode }) {
  const { Provider } = AppCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
