import { HexString, IVoucherDetails, ProgramMetadata, decodeAddress } from '@gear-js/api';
import { useAccount, useAlert, useApi } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { useBatchSignAndSend } from './use-batch-sign-and-send';
import { web3FromSource } from '@polkadot/extension-dapp';
import { KeyringPair } from '@polkadot/keyring/types';
import { sendTransaction } from '../utils';

type Session = {
  key: HexString;
  duration: number;
  allowedActions: string[];
};

function useCreateSession(programId: HexString, metadata: ProgramMetadata | undefined) {
  const { api, isApiReady } = useApi();
  const alert = useAlert();
  const { account } = useAccount();
  const { batchSignAndSend } = useBatchSignAndSend('all');
  const onError = (message: string) => alert.error(message);

  const isVoucherExpired = async ({ expiry }: IVoucherDetails) => {
    if (!isApiReady) throw new Error('API is not initialized');

    const { block } = await api.rpc.chain.getBlock();
    const currentBlockNumber = block.header.number.toNumber();

    return currentBlockNumber > expiry;
  };

  // TODO: reuse voucher from context
  const getLatestVoucher = async (address: string) => {
    if (!isApiReady) throw new Error('API is not initialized');

    const vouchers = await api.voucher.getAllForAccount(address, programId);

    const [entry] = Object.entries(vouchers).sort(
      ([, voucher], [, nextVoucher]) => nextVoucher.expiry - voucher.expiry,
    );

    if (!entry) return;
    const [id, voucher] = entry;

    return { ...voucher, id };
  };

  const getMessageExtrinsic = (payload: AnyJson) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');

    const destination = programId;
    const gasLimit = 250000000000; // TODO: replace with calculation after release fix

    return api.message.send({ destination, payload, gasLimit }, metadata);
  };

  const getVoucherExtrinsic = async (session: Session, voucherValue: number) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');
    if (!account) throw new Error('Account not found');

    const voucher = await getLatestVoucher(session.key);

    if (!voucher || account.decodedAddress !== voucher.owner) {
      const { extrinsic } = await api.voucher.issue(session.key, voucherValue, undefined, [programId]);
      return extrinsic;
    }

    const prolongDuration = api.voucher.minDuration; // TODO: need to consider session duration
    const balanceTopUp = voucherValue;

    return api.voucher.update(session.key, voucher.id, { prolongDuration, balanceTopUp });
  };
  type Options = Partial<{
    onSuccess: () => void;
    onError: (error: string) => void;
    onFinally: () => void;
    pair?: KeyringPair;
  }>;

  const createSession = async (
    session: Session,
    voucherValue: number,
    pairToSave: KeyringPair | null,
    { shouldIssueVoucher, vId, ...options }: Options & { shouldIssueVoucher: boolean } & { vId: string | void },
  ) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!account) throw new Error('Account not found');
    if (!metadata) throw new Error('Metadata not found');

    if (vId && pairToSave) {
      const { signer } = await web3FromSource(account.meta.source);
      const { signRaw } = signer;

      if (!signRaw) {
        throw new Error('signRaw is not a function');
      }

      const key = decodeAddress(pairToSave.address);
      const duration = session.duration;
      const allowedActions = session.allowedActions;

      const payloadToSign = { key, duration, allowedActions };

      if (!metadata.types?.others?.output) {
        console.log('no output');

        return;
      }

      const hexToSign = metadata.createType(metadata.types.others.output, payloadToSign).toHex();

      const { signature } = await signRaw({ address: account.address, data: hexToSign, type: 'bytes' });

      const messageExtrinsic = getMessageExtrinsic({
        CreateSession: { key: session.key, duration, allowedActions, signature },
      });

      const voucherExtrinsic = api.voucher.call(vId, { SendMessage: messageExtrinsic });

      await sendTransaction(voucherExtrinsic, pairToSave, ['UserMessageSent'], { ...options, onError });

      return;
    }

    const messageExtrinsic = getMessageExtrinsic({ CreateSession: session });

    const txs = shouldIssueVoucher
      ? [messageExtrinsic, await getVoucherExtrinsic(session, voucherValue)]
      : [messageExtrinsic];

    batchSignAndSend(txs, { ...options, onError });
  };

  const deleteSession = async (key: HexString, pair: KeyringPair, options: Options) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!account) throw new Error('Account not found');
    if (!metadata) throw new Error('Metadata not found');

    const messageExtrinsic = getMessageExtrinsic({
      DeleteSessionFromAccount: null,
    });

    const txs = [messageExtrinsic];

    const voucher = await getLatestVoucher(key); //if account has voucher
    if (!voucher) return batchSignAndSend(txs, { ...options, onError });

    const isOwner = account.decodedAddress === voucher.owner;
    const isExpired = await isVoucherExpired(voucher);

    if (!isExpired) {
      const declineExtrinsic = api.voucher.call(voucher.id, { DeclineVoucher: null });

      await sendTransaction(declineExtrinsic, pair, ['VoucherDeclined'], { ...options, onError });
    }

    if (isOwner) {
      const revokeExtrinsic = api.voucher.revoke(key, voucher.id);
      txs.push(revokeExtrinsic);
    }

    batchSignAndSend(txs, { ...options, onError });
  };

  return { createSession, deleteSession };
}

export { useCreateSession };
