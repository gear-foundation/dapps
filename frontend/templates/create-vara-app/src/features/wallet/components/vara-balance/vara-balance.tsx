import { useAccount, useApi, useBalanceFormat, useDeriveBalancesAll } from '@gear-js/react-hooks';

import VaraSVG from '../../assets/vara-coin.svg?react';

import styles from './vara-balance.module.css';

function VaraBalance() {
  const { account } = useAccount();
  const { isApiReady } = useApi();

  const { getFormattedBalance } = useBalanceFormat();
  const balances = useDeriveBalancesAll(account?.decodedAddress);
  const balance = isApiReady && balances ? getFormattedBalance(balances.freeBalance) : undefined;

  if (!balance) return null;

  return (
    <div className={styles.balance}>
      <VaraSVG />

      <p className={styles.text}>
        <span className={styles.value}>{balance.value}</span>
        <span className={styles.unit}>{balance.unit}</span>
      </p>
    </div>
  );
}

export { VaraBalance };
