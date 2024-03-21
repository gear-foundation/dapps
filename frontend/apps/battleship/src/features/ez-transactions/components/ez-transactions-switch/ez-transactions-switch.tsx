import { EnableSignlessSession } from '@/features/signless-transactions';
import { EnableGaslessSession } from '@/features/gasless-transactions';

import { useEzTransactions } from '../../context';
import styles from './ez-transactions-switch.module.css';

function EzTransactionsSwitch() {
  const { gasless, signless } = useEzTransactions();

  return (
    <div className={styles.container}>
      <EnableGaslessSession
        type="switcher"
        disabled={signless.isSessionActive}
        message={signless.isSessionActive ? 'Signless Session is Active' : ''}
      />

      <EnableSignlessSession
        type="switcher"
        onSessionCreate={signless.onSessionCreate}
        shouldIssueVoucher={!gasless.isEnabled}
        disabled={!signless.isSessionActive && gasless.isActive}
        message={!signless.isSessionActive && gasless.isActive ? 'Gasless Session is Active' : ''}
      />
    </div>
  );
}

export { EzTransactionsSwitch };
