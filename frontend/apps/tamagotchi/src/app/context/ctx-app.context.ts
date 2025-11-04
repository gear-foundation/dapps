import { createContext, Dispatch, SetStateAction, useContext } from 'react';

export type AppContextValue = {
  isPending: boolean;
  setIsPending: Dispatch<SetStateAction<boolean>>;
};

export const AppCtx = createContext<AppContextValue | undefined>(undefined);

export function useApp() {
  const context = useContext(AppCtx);

  if (!context) {
    throw new Error('useApp must be used within an AppProvider');
  }

  return context;
}
