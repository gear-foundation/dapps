import { ReactElement } from 'react';

export type WalletSwitchProps = {
  children: ReactElement;
  onChainChange: (newChain: string) => void;
};
