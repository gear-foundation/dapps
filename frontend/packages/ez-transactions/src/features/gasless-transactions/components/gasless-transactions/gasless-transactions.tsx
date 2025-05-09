import { useAccount } from '@gear-js/react-hooks';

import GaslessSVG from '@ez/assets/icons/gas-station-line.svg?react';

import { useGaslessTransactions } from '../../context';
import { EnableGaslessSession } from '../enable-gasless-session';

import styles from './gasless-transactions.module.css';

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

            <h3 className={styles.title}>{isActive ? 'Gasless Session is active' : 'Gasless Session is enabled'}</h3>
          </div>

          <EnableGaslessSession type="button" disabled={disabled} />
        </div>
      )}

      {!isEnabled && <EnableGaslessSession type="button" disabled={disabled} />}
    </div>
  ) : null;
}

export { GaslessTransactions };
