import { SignlessTransactions } from '@dapps-frontend/signless-transactions';
import { useEzTransactions } from '../../context';

type Props = {
  allowedActions: string[];
};

function EzSignlessTransactions({ allowedActions }: Props) {
  const { gasless, signless } = useEzTransactions();

  return (
    <SignlessTransactions
      allowedActions={allowedActions}
      onSessionCreate={signless.onSessionCreate}
      shouldIssueVoucher={!gasless.isEnabled}
      disabled={!signless.isSessionActive && gasless.isActive}
      requiredBalance={gasless.isEnabled ? 0 : undefined}
      bindedSessionDuration={gasless.isEnabled ? gasless.voucherStatus?.duration : undefined}
    />
  );
}

export { EzSignlessTransactions };
