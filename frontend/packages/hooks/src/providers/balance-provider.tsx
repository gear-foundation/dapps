import { Dispatch, PropsWithChildren, SetStateAction, createContext, useState } from 'react';

type AvailableBalance = undefined | { value: string; unit: string; existentialDeposit: string };

export type BalanceContextProps = {
  isAvailableBalanceReady: boolean;
  availableBalance: AvailableBalance;
  setIsAvailableBalanceReady: Dispatch<SetStateAction<boolean>>;
  setAvailableBalance: Dispatch<SetStateAction<AvailableBalance>>;
};

export const BalanceContext = createContext<Partial<BalanceContextProps>>({});

export function BalanceProvider({ children }: PropsWithChildren) {
  const [isAvailableBalanceReady, setIsAvailableBalanceReady] = useState<boolean>(false);
  const [availableBalance, setAvailableBalance] = useState<AvailableBalance>(undefined);

  return (
    <BalanceContext.Provider
      value={{ availableBalance, isAvailableBalanceReady, setAvailableBalance, setIsAvailableBalanceReady }}>
      {children}
    </BalanceContext.Provider>
  );
}
