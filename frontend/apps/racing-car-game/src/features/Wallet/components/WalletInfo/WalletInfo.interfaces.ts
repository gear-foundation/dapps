import { Account } from '@gear-js/react-hooks';

export type WalletInfoProps = {
  account?: Account;
  withoutBalance?: boolean;
  buttonClassName?: string;
};
