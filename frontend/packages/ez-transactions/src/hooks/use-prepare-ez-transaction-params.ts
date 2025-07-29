import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { IKeyringPair } from '@polkadot/types/types';

import { useEzTransactions } from '../context';

type PrepareEzTransactionParamsResult = {
  sessionForAccount: HexString | null;
  account: { addressOrPair: IKeyringPair } | undefined;
  voucherId: HexString | undefined;
  gasLimit: { increaseGas: number };
};

const usePrepareEzTransactionParams = () => {
  const { account } = useAccount();
  const { signless, gasless } = useEzTransactions();
  const { pair, voucher } = signless;

  const prepareEzTransactionParams = async (
    sendFromBaseAccount?: boolean,
  ): Promise<PrepareEzTransactionParamsResult> => {
    if (!account) throw new Error('Account not found');

    const sendFromPair = pair && voucher?.id && !sendFromBaseAccount;
    const sessionForAccount = sendFromPair ? account.decodedAddress : null;

    let voucherId = sendFromPair ? voucher?.id : gasless.voucherId;
    if (account && gasless.isEnabled && !gasless.voucherId && (sendFromBaseAccount || !signless.isActive)) {
      voucherId = await gasless.requestVoucher(account.address);
    }

    return {
      sessionForAccount,
      account: sendFromPair ? { addressOrPair: pair } : undefined,
      voucherId,
      gasLimit: { increaseGas: 10 },
    };
  };

  return { prepareEzTransactionParams };
};

export { usePrepareEzTransactionParams, type PrepareEzTransactionParamsResult };
