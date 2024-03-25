import { SignlessTransactions } from '@/features/signless-transactions';

import { useEzTransactions } from '../../context';

function EzSignlessTransactions() {
  const { gasless, signless } = useEzTransactions();

  return (
    <SignlessTransactions
      onSessionCreate={signless.onSessionCreate}
      shouldIssueVoucher={!gasless.isEnabled}
      disabled={!signless.isSessionActive && gasless.isActive}
      requiredBalance={gasless.isEnabled ? 11 : undefined}
    />
  );
}

export { EzSignlessTransactions };
