import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { useAccount } from '@gear-js/react-hooks';
import { web3FromSource } from '@polkadot/extension-dapp';
import { useProgram } from './sails';

const usePrepareEzTransactionParams = () => {
  const gasLimit = 250_000_000_000n;
  const program = useProgram();
  const { account } = useAccount();
  const { signless, gasless } = useEzTransactions();
  const { pair, voucher } = signless;

  const prepareEzTransactionParams = async () => {
    if (!program) throw new Error('program does not found');
    if (!account) throw new Error('Account not found');
    const sessionForAccount = pair ? account.decodedAddress : null;

    let voucherId = voucher?.id || gasless.voucherId;
    if (account && gasless.isEnabled && !gasless.voucherId && !signless.isActive) {
      voucherId = await gasless.requestVoucher(account.address);
    }

    const injector = await web3FromSource(account.meta.source);

    return {
      sessionForAccount,
      account: pair
        ? { addressOrPair: pair }
        : { addressOrPair: account.decodedAddress, signerOptions: { signer: injector.signer } },
      voucherId,
      gasLimit,
    };
  };

  return { prepareEzTransactionParams };
};

export { usePrepareEzTransactionParams };
