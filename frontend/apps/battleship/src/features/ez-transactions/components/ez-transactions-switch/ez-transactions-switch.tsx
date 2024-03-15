import { EnableSession as SignlessSessionSwitch } from '@/features/signless-transactions';
import { EnableSession as GaslessSessionSwitch } from '@/features/gasless-transactions';

import { useEzTransactions } from '../../context';
import styles from './ez-transactions-switch.module.scss';

function EzTransactionsSwitch() {
  const { signless } = useEzTransactions();

  return (
    <div className={styles.container}>
      <GaslessSessionSwitch type="switcher" disabled={signless.isActive} />
      <SignlessSessionSwitch type="switcher" onSessionCreate={signless.onSessionCreate} />
    </div>
  );
}

export { EzTransactionsSwitch };
