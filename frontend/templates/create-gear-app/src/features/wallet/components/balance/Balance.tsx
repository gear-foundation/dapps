import { useAccount, useApi, useBalanceFormat, useDeriveBalancesAll } from '@gear-js/react-hooks';

import styles from './Balance.module.scss';

function Balance() {
  const { isApiReady } = useApi();
  const { account } = useAccount();
  const balances = useDeriveBalancesAll(account?.address);
  const { getFormattedBalance } = useBalanceFormat();

  const balance = balances?.freeBalance;
  const formattedBalance = isApiReady && balance ? getFormattedBalance(balance) : undefined;

  return formattedBalance ? (
    <div>
      <p className={styles.heading}>Balance:</p>
      <p>
        <span className={styles.value}>{formattedBalance.value}</span>
        <span className={styles.unit}>{formattedBalance.unit}</span>
      </p>
    </div>
  ) : null;
}

export { Balance };
