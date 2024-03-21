import { useAccount } from '@gear-js/react-hooks';
import styles from './gasless-transactions.module.css';
import { EnableGaslessSession } from '../enable-gasless-session';
import { useGaslessTransactions } from '../..';
import { ReactComponent as GaslessSVG } from '../../assets/icons/gas-station-line.svg';

type Props = {
  disabled?: boolean;
};

function GaslessTransactions({ disabled }: Props) {
  const { account } = useAccount();
  const { isEnabled, isActive } = useGaslessTransactions();

  return account ? (
    <div className={styles.container}>
      {isEnabled && (
        <div className={styles.sessionContainer}>
          <div className={styles.titleWrapper}>
            <GaslessSVG />

            <h3 className={styles.title}>
              {isActive ? 'Gasless Session is active' : 'Gasless Session will start with the first game'}
            </h3>
          </div>

          <EnableGaslessSession type="button" />
        </div>
      )}

      {!isEnabled && <EnableGaslessSession type="button" disabled={disabled} />}
    </div>
  ) : null;
}

export { GaslessTransactions };
