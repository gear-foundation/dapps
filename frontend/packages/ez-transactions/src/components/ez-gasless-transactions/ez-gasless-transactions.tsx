import { GaslessTransactions } from '@dapps-frontend/gasless-transactions';
import { useEzTransactions } from '../../context';

function EzGaslessTransactions() {
  const { signless } = useEzTransactions();

  return <GaslessTransactions disabled={signless.isSessionActive} />;
}

export { EzGaslessTransactions };
