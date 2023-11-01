import { useApi, useAccount, useBalance, useBalanceFormat } from '@gear-js/react-hooks';
import { useFTBalance } from 'hooks/api';
import { AccountButton } from '../account-button';
import styles from './Wallet.module.scss';

type Props = {
  address: string;
  name: string | undefined;
  onClick: () => void;
};

function Wallet({ address, name, onClick }: Props) {
  const { isApiReady } = useApi();
  const { account } = useAccount();
  const { balance } = useBalance(account?.address);
  const { getFormattedBalance } = useBalanceFormat();
  const formattedBalance = isApiReady && balance ? getFormattedBalance(balance) : undefined;

  const tokens = useFTBalance();

  return (
    <div className={styles.wallet}>
      {tokens !== undefined && (
        <p className={styles.balance}>
          {tokens} <span className={styles.currency}>Tokens</span>
        </p>
      )}

      <p className={styles.balance}>
        {formattedBalance?.value} <span className={styles.currency}>{formattedBalance?.unit}</span>
      </p>

      <AccountButton address={address} name={name} onClick={onClick} />
    </div>
  );
}

export { Wallet };
