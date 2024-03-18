import { SignlessTransactions } from '@dapps-frontend/signless-transactions';

import { useEzTransactions } from '../../context';

function EzSignlessTransactions() {
  const { gasless, signless } = useEzTransactions();

  return <SignlessTransactions onSessionCreate={signless.onSessionCreate} shouldIssueVoucher={!gasless.isEnabled} />;
}

export { EzSignlessTransactions };
