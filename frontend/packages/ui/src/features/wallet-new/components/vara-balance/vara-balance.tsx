import { useAccount, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { ReactComponent as VaraSVG } from '../../assets/vara.svg';
import { useFreeAccountBalance } from '../../hooks';
import styles from './vara-balance.module.css';

function VaraBalance() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  const { getFormattedBalance } = useBalanceFormat();
  const { freeAccountBalance } = useFreeAccountBalance();
  const balance = isApiReady && freeAccountBalance ? getFormattedBalance(freeAccountBalance) : undefined;

  return isAccountReady && balance ? (
    <div className={styles.balance}>
      <VaraSVG />

      <p className={styles.text}>
        <span className={styles.value}>{balance.value}</span>
        <span className={styles.unit}>{balance.unit}</span>
      </p>
    </div>
  ) : null;
}

export { VaraBalance };
