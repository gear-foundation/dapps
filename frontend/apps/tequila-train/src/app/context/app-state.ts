import { useState } from 'react';

function useAppState() {
  const [isPending, setIsPending] = useState(false);
  const [isAllowed, setIsAllowed] = useState(false);
  const [openEmptyPopup, setOpenEmptyPopup] = useState(false);
  const [openWinnerPopup, setOpenWinnerPopup] = useState(false);
  const [isUserCancelled, setIsUserCancelled] = useState(false);

  return {
    isPending,
    setIsPending,
    isAllowed,
    setIsAllowed,
    openEmptyPopup,
    setOpenEmptyPopup,
    openWinnerPopup,
    setOpenWinnerPopup,
    isUserCancelled,
    setIsUserCancelled,
  };
}

type AppContextValue = ReturnType<typeof useAppState>;

export { useAppState };
export type { AppContextValue };
