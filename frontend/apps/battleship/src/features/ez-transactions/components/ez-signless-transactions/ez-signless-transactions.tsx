import { SignlessTransactions } from '@/features/signless-transactions';

import { useEzTransactions } from '../../context';

function EzSignlessTransactions() {
  const { signless } = useEzTransactions();

  return <SignlessTransactions onSessionCreate={signless.onSessionCreate} />;
}

export { EzSignlessTransactions };
