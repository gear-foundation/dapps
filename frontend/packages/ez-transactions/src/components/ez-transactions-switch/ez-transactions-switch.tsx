import { EnableSignlessSession } from '@dapps-frontend/signless-transactions';
import { EnableGaslessSession } from '@dapps-frontend/gasless-transactions';

import { useEzTransactions } from '../../context';
import styles from './ez-transactions-switch.module.css';

function EzTransactionsSwitch() {
  const { gasless, signless } = useEzTransactions();

  return (
    <div className={styles.container}>
      <EnableGaslessSession type="switcher" disabled={signless.isActive} />

      <EnableSignlessSession
        type="switcher"
        onSessionCreate={signless.onSessionCreate}
        shouldIssueVoucher={!gasless.isEnabled}
      />
    </div>
  );
}

export { EzTransactionsSwitch };
