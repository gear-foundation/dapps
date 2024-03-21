import { GaslessTransactions } from '@/features/gasless-transactions';
import { useEzTransactions } from '../../context';

function EzGaslessTransactions() {
  const { signless } = useEzTransactions();

  return <GaslessTransactions disabled={signless.isSessionActive} />;
}

export { EzGaslessTransactions };
