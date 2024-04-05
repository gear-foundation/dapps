import { GaslessTransactions } from '@dapps-frontend/gasless-transactions';
import { useEzTransactions } from '../../context';

type Props = {
  disabled?: boolean;
  disabledTurnOn?: boolean;
};

function EzGaslessTransactions({ disabled, disabledTurnOn }: Props) {
  const { signless } = useEzTransactions();

  return <GaslessTransactions disabled={signless.isSessionActive || disabled} disabledTurnOn={disabledTurnOn} />;
}

export { EzGaslessTransactions };
