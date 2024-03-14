import { useAccount } from '@gear-js/react-hooks';
import styles from './gasless-transactions.module.css';
import { EnableSession } from '../enable-session';
import { useGaslessTransactions } from '../..';
import { ReactComponent as GaslessSVG } from '../../assets/icons/gas-station-line.svg';

function GaslessTransactions() {
  const { account } = useAccount();
  const { voucherId, isEnabled } = useGaslessTransactions();

  return account ? (
    <div className={styles.container}>
      {isEnabled && (
        <div className={styles.sessionContainer}>
          <div className={styles.titleWrapper}>
            <GaslessSVG />
            <h3 className={styles.title}>
              {voucherId ? 'Gasless Session is active' : 'Gasless Session will start with the first game'}
            </h3>
          </div>
          <EnableSession type="button" />
        </div>
      )}

      {!isEnabled && <EnableSession type="button" />}
    </div>
  ) : null;
}

export { GaslessTransactions };
