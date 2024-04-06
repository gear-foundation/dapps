import { useAccount } from '@gear-js/react-hooks';
import styles from './gasless-transactions.module.css';
import { EnableGaslessSession } from '../enable-gasless-session';
import { useGaslessTransactions } from '../..';
import { ReactComponent as GaslessSVG } from '../../assets/icons/gas-station-line.svg';

type Props = {
  disabled?: boolean;
  disabledTurnOn?: boolean;
};

function GaslessTransactions({ disabled, disabledTurnOn }: Props) {
  const { account } = useAccount();
  const { isEnabled, isActive } = useGaslessTransactions();

  return account ? (
    <div className={styles.container}>
      {isEnabled && (
        <div className={styles.sessionContainer}>
          <div className={styles.titleWrapper}>
            <GaslessSVG />

            <h3 className={styles.title}>{isActive ? 'Gasless Session is active' : 'Gasless Session is enabled'}</h3>
          </div>

          <EnableGaslessSession type="button" disabled={disabled} />
        </div>
      )}

      {!isEnabled && <EnableGaslessSession type="button" disabled={disabled} disabledTurnOn={disabledTurnOn} />}
    </div>
  ) : null;
}

export { GaslessTransactions };
