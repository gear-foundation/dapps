import { EnableSession as SignlessSessionSwitch } from '@/features/signless-transactions';
import { EnableSession as GaslessSessionSwitch } from '@/features/gasless-transactions';

import styles from './ez-transactions-switch.module.scss';

function EzTransactionsSwitch() {
  return (
    <div className={styles.container}>
      <GaslessSessionSwitch type="switcher" />
      <SignlessSessionSwitch type="switcher" />
    </div>
  );
}

export { EzTransactionsSwitch };
