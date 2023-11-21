import { useBalance, useBalanceFormat } from '@gear-js/react-hooks';
import { AccountButton } from '../account-button';
import styles from './Wallet.module.scss';

type Props = {
  address: string;
  name: string | undefined;
  onClick: () => void;
};

function Wallet({ address, name, onClick }: Props) {
  const { balance } = useBalance(address);
  const { getFormattedBalance } = useBalanceFormat();
  const formattedBalance = balance ? getFormattedBalance(balance) : undefined;

  return (
    <div className={styles.wallet}>
      <p className={styles.balance}>
        {formattedBalance?.value} <span className={styles.currency}>{formattedBalance?.unit}</span>
      </p>
      <AccountButton address={address} name={name} onClick={onClick} />
    </div>
  );
}

export { Wallet };
