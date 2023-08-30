import { createContext, ReactNode, useState } from "react";

export const AppCtx = createContext({} as ReturnType<typeof useProgram>);

const useProgram = () => {
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
    setOpenWinnerPopup
  };
};

export function AppProvider({ children }: { children: ReactNode }) {
  const { Provider } = AppCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
