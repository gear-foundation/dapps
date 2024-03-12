import { HexString, ProgramMetadata } from '@gear-js/api';
import { useAccount, useAlert, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { useBatchSignAndSend } from './use-batch-sign-and-send';
import { KeyringPair } from '@polkadot/keyring/types';
import { sendTransaction } from '@/utils';
import { useGaslessTransactions } from '@dapps-frontend/gasless-transactions';

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
  const { isActive: isGaslessActive, checkAndFetchVoucher } = useGaslessTransactions();

  const onError = (message: string) => alert.error(message);

  const getMessage = (payload: AnyJson, externalAccount?: HexString) => {
    const destination = programId;
    // TODO: replace with calculation after release fix
    const gasLimit = 250000000000;

    return { destination, payload, gasLimit, account: externalAccount || account?.decodedAddress };
  };

  const deleteSession = async (key: HexString, pair?: KeyringPair, _options?: Options) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');

    const vouchersForAccount = await api.voucher.getAllForAccount(key, programId);

    const accountVoucherId = Object.keys(vouchersForAccount)[0];

    const details = await api?.voucher.getDetails(key, accountVoucherId as `0x${string}`);
    const finilizedBlockHash = await api?.blocks.getFinalizedHead();
    const currentBlockNumber = await api.blocks.getBlockNumber(finilizedBlockHash.toHex());

    const isExpired = currentBlockNumber.toNumber() > details.expiry;

    if (!isExpired && pair) {
      const declineExtrrinsic = api.voucher.call(accountVoucherId, { DeclineVoucher: null });

      await sendTransaction(declineExtrrinsic, pair, ['VoucherDeclined']);
    }

    const revokeExtrrinsic = api.voucher.revoke(pair?.address as string, accountVoucherId);

    const message = getMessage({ DeleteSessionFromAccount: null });
    const extrinsic = api.message.send(message, metadata);
    console.log('details.owner === account?.decodedAddress');
    console.log(details.owner === account?.decodedAddress);
    const txs = details.owner === account?.decodedAddress ? [extrinsic, revokeExtrrinsic] : [extrinsic];

    batchSignAndSend(txs, { ..._options, onError });
  };

  const createSession = async (session: Session, pair: KeyringPair, voucherValue: number, _options: Options) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');
    if (!account) throw new Error('Account not found');

    const minDuration = api.voucher.minDuration;

    const vouchersForAccount = await api.voucher.getAllForAccount(pair.address, programId);
    const accountVoucherId = Object.keys(vouchersForAccount)[0];

    if (!accountVoucherId && isGaslessActive) {
      const createdVoucherId = await checkAndFetchVoucher(pair.address);

      if (createdVoucherId) {
        console.log(session);
        const message = getMessage({ CreateSession: session }, session.key);
        const extrinsic = api.message.send(message, metadata);
        const voucherExtrinsic = api.voucher.call(createdVoucherId as string, { SendMessage: extrinsic });

        await sendTransaction(voucherExtrinsic, pair, ['UserMessageSent'], { ..._options, onError });

        return;
      }
    }

    if (accountVoucherId) {
      const message = getMessage({ CreateSession: session }, session.key);
      const extrinsic = api.message.send(message, metadata);
      const voucherExtrinsic = api.voucher.call(accountVoucherId as string, { SendMessage: extrinsic });

      await sendTransaction(voucherExtrinsic, pair, ['UserMessageSent'], { ..._options, onError });

      return;
    }

    const message = getMessage({ CreateSession: session });
    const extrinsic = api.message.send(message, metadata);

    const voucher = await api.voucher.issue(session.key, voucherValue, minDuration, [programId], true);

    const txs = [extrinsic, voucher.extrinsic];
    const options = { ..._options, onError };

    batchSignAndSend(txs, options);
  };

  const updateSession = async (session: Session, pair: KeyringPair, voucherValue: number, _options: Options) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');
    if (!account) throw new Error('Account not found');

    const updateVoucher = async (accountVoucherId: string) => {
      const details = await api?.voucher.getDetails(session.key, accountVoucherId as `0x${string}`);

      const finilizedBlockHash = await api?.blocks.getFinalizedHead();
      const currentBlockNumber = await api?.blocks.getBlockNumber(finilizedBlockHash.toHex());

      const isNeedProlongDuration = currentBlockNumber.toNumber() > details.expiry;

      if (voucherValue || isNeedProlongDuration) {
        if (!isGaslessActive) {
          const minDuration = api.voucher.minDuration;

          const voucherExtrinsic = await api.voucher.update(session.key, accountVoucherId, {
            balanceTopUp: voucherValue
              ? Number(getFormattedBalance(balance.toNumber()).value) + Number(getFormattedBalance(voucherValue).value)
              : undefined,
            prolongDuration: isNeedProlongDuration ? minDuration : undefined,
          });

          return voucherExtrinsic;
        }
      }

      return null;
    };

    const vouchersForAccount = await api.voucher.getAllForAccount(pair.address, programId);
    const accountVoucherId = Object.keys(vouchersForAccount)[0];

    console.log('accountVoucherId');
    console.log(accountVoucherId);
    console.log('pair.address');
    console.log(pair.address);
    console.log('session.key');
    console.log(session.key);

    if (!accountVoucherId && isGaslessActive) {
      const voucherId = await checkAndFetchVoucher(pair.address);

      if (voucherId) {
        const message = getMessage({ CreateSession: session }, session.key);
        const extrinsic = api.message.send(message, metadata);
        const voucherExtrinsic = api.voucher.call(voucherId as string, { SendMessage: extrinsic });

        await sendTransaction(voucherExtrinsic, pair, ['UserMessageSent'], { ..._options, onError });
        return;
      }
    }

    if (accountVoucherId) {
      const message = getMessage({ CreateSession: session }, session.key);
      const extrinsic = api.message.send(message, metadata);
      const voucherExtrinsic = api.voucher.call(accountVoucherId as string, { SendMessage: extrinsic });
      console.log('voucherExtrinsic');
      console.log(voucherExtrinsic);
      await sendTransaction(voucherExtrinsic, pair, ['UserMessageSent'], { ..._options, onError });

      return;
    }

    const message = getMessage({ CreateSession: session });

    const extrinsic = await api.message.send(message, metadata);

    const balance = await api.balance.findOut(accountVoucherId);

    const updatedVoucherExtrinsic = await updateVoucher(accountVoucherId);

    const txs = updatedVoucherExtrinsic ? [extrinsic, updatedVoucherExtrinsic] : [extrinsic];
    const options = { ..._options, onError };

    batchSignAndSend(txs, options);
  };

  return { createSession, updateSession, deleteSession };
}

export { useCreateSession };
