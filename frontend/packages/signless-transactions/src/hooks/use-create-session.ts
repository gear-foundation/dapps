import { HexString, ProgramMetadata } from '@gear-js/api';
import { useAccount, useAlert, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { useBatchSignAndSend } from './use-batch-sign-and-send';
import { KeyringPair } from '@polkadot/keyring/types';
import { sendTransaction } from '@/utils';

type Session = {
  key: HexString;
  duration: number;
  allowedActions: string[];
};

type Options = {
  onSuccess: () => void;
  onFinally: () => void;
};

function useCreateSession(programId: HexString, metadata: ProgramMetadata | undefined) {
  const { api, isApiReady } = useApi();
  const alert = useAlert();
  const { account } = useAccount();
  const { batchSignAndSend } = useBatchSignAndSend('all');
  const { getFormattedBalance } = useBalanceFormat();

  const onError = (message: string) => alert.error(message);

  const getMessage = (payload: AnyJson) => {
    const destination = programId;
    // TODO: replace with calculation after release fix
    const gasLimit = 250000000000;

    return { destination, payload, gasLimit };
  };

  const deleteSession = async (key: HexString, pair?: KeyringPair) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');

    const message = getMessage({ DeleteSessionFromAccount: null });
    const extrinsic = api.message.send(message, metadata);

    const vouchersForAccount = await api.voucher.getAllForAccount(key, programId);

    const accountVoucherId = Object.keys(vouchersForAccount)[0];

    const declineExtrrinsic = api.voucher.call(accountVoucherId, { DeclineVoucher: null });

    if (pair) {
      await sendTransaction(declineExtrrinsic, pair, ['VoucherDeclined']);
    }

    const revokeExtrrinsic = api.voucher.revoke(key, accountVoucherId);

    const txs = [extrinsic, revokeExtrrinsic];

    batchSignAndSend(txs, { onError });
  };

  const createSession = async (session: Session, voucherValue: number, _options: Options) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');
    if (!account) throw new Error('Account not found');

    const message = getMessage({ CreateSession: session });

    const extrinsic = api.message.send(message, metadata);

    const minDuration = api.voucher.minDuration;

    const voucher = await api.voucher.issue(session.key, voucherValue, minDuration, [programId], true);

    const txs = [extrinsic, voucher.extrinsic];
    const options = { ..._options, onError };

    batchSignAndSend(txs, options);
  };

  const updateSession = async (session: Session, voucherValue: number, _options: Options) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');
    if (!account) throw new Error('Account not found');

    const updateVoucher = async (accountVoucherId: string) => {
      const details = await api?.voucher.getDetails(session.key, accountVoucherId as `0x${string}`);

      const finilizedBlockHash = await api?.blocks.getFinalizedHead();
      const currentBlockNumber = await api.blocks.getBlockNumber(finilizedBlockHash.toHex());

      const isNeedProlongDuration = currentBlockNumber.toNumber() > details.expiry;

      if (voucherValue || isNeedProlongDuration) {
        const minDuration = api.voucher.minDuration;

        const voucherExtrinsic = await api.voucher.update(session.key, accountVoucherId, {
          balanceTopUp: voucherValue
            ? Number(getFormattedBalance(balance.toNumber()).value) + Number(getFormattedBalance(voucherValue).value)
            : undefined,
          prolongDuration: isNeedProlongDuration ? minDuration : undefined,
        });

        return voucherExtrinsic;
      }

      return null;
    };

    const message = getMessage({ CreateSession: session });

    const extrinsic = await api.message.send(message, metadata);

    const vouchersForAccount = await api.voucher.getAllForAccount(session.key, programId);

    const accountVoucherId = Object.keys(vouchersForAccount)[0];

    const balance = await api.balance.findOut(accountVoucherId);

    const updatedVoucherExtrinsic = await updateVoucher(accountVoucherId);

    const txs = updatedVoucherExtrinsic ? [extrinsic, updatedVoucherExtrinsic] : [extrinsic];
    const options = { ..._options, onError };

    batchSignAndSend(txs, options);
  };

  return { createSession, updateSession, deleteSession };
}

export { useCreateSession };
