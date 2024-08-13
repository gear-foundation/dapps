import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { useAccount } from '@gear-js/react-hooks';
import { TransactionBuilder } from 'sails-js';

const useMakeTransaction = () => {
  const { account } = useAccount();

  const { gasless, signless } = useEzTransactions();

  return async (transactrionBuilder: TransactionBuilder<null>) => {
    if (!account?.decodedAddress) {
      throw new Error('No account found!');
    }

    let { voucherId } = gasless;
    if (account && gasless.isEnabled && !gasless.voucherId && !signless.isActive) {
      voucherId = await gasless.requestVoucher(account.address);
    }

    const transaction = transactrionBuilder.withAccount(account.address, { signer: account.signer });

    if (voucherId) {
      transaction.withVoucher(voucherId);
    }

    return transaction;
  };
};

export { useMakeTransaction };
