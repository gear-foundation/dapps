import { useAccount } from '@gear-js/react-hooks';
import styles from './gasless-transactions.module.css';
import { EnableSession } from '../enable-session';
import { useGaslessTransactions } from '../..';
import { ReactComponent as GaslessSVG } from '../../assets/icons/gas-station-line.svg';

function GaslessTransactions() {
  const { account } = useAccount();
  const { isActive } = useGaslessTransactions();

  return account ? (
    <div className={styles.container}>
      {isActive && (
        <>
          <div className={styles.sessionContainer}>
            <div className={styles.titleWrapper}>
              <GaslessSVG />
              <h3 className={styles.title}>Gasless Session is active</h3>
            </div>
            <EnableSession type="button" />
          </div>
        </>
      )}
      {!isActive && <EnableSession type="button" />}
    </div>
  ) : null;
}

export { GaslessTransactions };
