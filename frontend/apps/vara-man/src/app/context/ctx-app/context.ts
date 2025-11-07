import { createContext, useContext } from 'react';
import type { Dispatch, SetStateAction } from 'react';

export type AppContextValue = {
  isPending: boolean;
  setIsPending: Dispatch<SetStateAction<boolean>>;
};

const AppContext = createContext<AppContextValue | undefined>(undefined);

function useApp() {
  const context = useContext(AppContext);

  if (!context) {
    throw new Error('useApp must be used within an AppProvider');
  }

  return context;
}

export { AppContext, useApp };
