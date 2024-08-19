import { HexString, IVoucherDetails, ProgramMetadata, decodeAddress } from '@gear-js/api';
import { Account, useAccount, useAlert, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { AnyJson } from '@polkadot/types/types';
import { useBatchSignAndSend } from './use-batch-sign-and-send';
import { KeyringPair } from '@polkadot/keyring/types';
import { sendTransaction } from '../utils';
import { useIsAvailable } from '.';

type Session = {
  key: HexString;
  duration: number;
  allowedActions: string[];
};

function useCreateSession(
  programId: HexString,
  metadata: ProgramMetadata | undefined,
  createSignatureType?: (metadata: ProgramMetadata, payloadToSig: Session) => `0x${string}`,
) {
  const { api, isApiReady } = useApi();
  const alert = useAlert();
  const { account } = useAccount();
  const { getFormattedBalanceValue } = useBalanceFormat();
  const minRequiredBalanceToDeleteSession =
    getFormattedBalanceValue(api?.existentialDeposit.toNumber() || 0).toNumber() + 5;
  const isDeleteSessionAvailable = useIsAvailable(minRequiredBalanceToDeleteSession, false);
  const { batchSignAndSend, batchSign, batchSend } = useBatchSignAndSend('all');
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

  const getDurationBlocks = (durationMS: number) => {
    if (!api) {
      return;
    }

    const blockTimeMs = api.consts.babe.expectedBlockTime.toNumber();

    return (durationMS / blockTimeMs) * 1.05; // +5% to cover transaction time
  };

  const getVoucherExtrinsic = async (session: Session, voucherValue: number) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');
    if (!account) throw new Error('Account not found');

    const voucher = await getLatestVoucher(session.key);
    const durationBlocks = getDurationBlocks(session.duration);

    if (!voucher || account.decodedAddress !== voucher.owner) {
      const { extrinsic } = await api.voucher.issue(session.key, voucherValue, durationBlocks, [programId]);
      return extrinsic;
    }

    const prolongDuration = durationBlocks;
    const balanceTopUp = voucherValue;

    return api.voucher.update(session.key, voucher.id, { prolongDuration, balanceTopUp });
  };
  type Options = {
    onSuccess: () => void;
    onFinally: () => void;
  };

  type CreeateSessionOptions = {
    pair?: KeyringPair;
    voucherId?: `0x${string}`;
    shouldIssueVoucher: boolean;
  };

  const getAccountSignature = async (metadata: ProgramMetadata, account: Account, payloadToSign: Session) => {
    const { signer } = account;
    const { signRaw } = signer;

    console.log('SIGNATURE:');
    console.log('METADATA');
    console.log(metadata);
    console.log('ACCOUNT');
    console.log(account);
    console.log('PAYLOAD_TO_SIGN');
    console.log(payloadToSign);
    console.log('SIGNER');
    console.log(signer);
    console.log('signRaw');
    console.log(signRaw);

    if (!signRaw) {
      throw new Error('signRaw is not a function');
    }

    if (metadata.types?.others?.output === null) {
      throw new Error(`Metadata type doesn't exist`);
    }

    console.log('metadata.types?.others?.output');
    console.log(metadata.types?.others?.output);

    const hexToSign = createSignatureType
      ? createSignatureType(metadata, payloadToSign)
      : metadata.createType(metadata.types.others.output, payloadToSign).toHex();

    console.log('HEXtoSIGN: ', hexToSign);
    return signRaw({ address: account.address, data: hexToSign, type: 'bytes' });
  };

  const createSession = async (
    session: Session,
    voucherValue: number,
    { shouldIssueVoucher, voucherId, pair, ...options }: Options & CreeateSessionOptions,
  ) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!account) throw new Error('Account not found');
    if (!metadata) throw new Error('Metadata not found');

    if (voucherId && pair) {
      const { signature } = await getAccountSignature(metadata, account, {
        ...session,
        key: decodeAddress(pair.address),
      });

      const messageExtrinsic = getMessageExtrinsic({
        CreateSession: { ...session, signature },
      });

      const voucherExtrinsic = api.voucher.call(voucherId, { SendMessage: messageExtrinsic });

      await sendTransaction(voucherExtrinsic, pair, ['UserMessageSent'], { ...options, onError });

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

    if (!isDeleteSessionAvailable) {
      alert.error('Low balance on account to disable session');
      options.onFinally();
      throw new Error('Low balance on account to disable session');
    }

    const messageExtrinsic = getMessageExtrinsic({
      DeleteSessionFromAccount: null,
    });

    const txs = [messageExtrinsic];

    const voucher = await getLatestVoucher(key);
    if (!voucher) return batchSignAndSend(txs, { ...options, onError });

    const isOwner = account.decodedAddress === voucher.owner;
    const isExpired = await isVoucherExpired(voucher);

    if (isOwner) {
      // revoke on onSuccess is not called to avoid second signatures popup
      const revokeExtrinsic = api.voucher.revoke(key, voucher.id);
      txs.push(revokeExtrinsic);
    }

    // We need to sign transactions before sending declineExtrinsic;
    // Otherwise, if the signing is canceled, the voucher will be invalid.
    const signedTxs = await batchSign(txs, options);

    if (!signedTxs) {
      throw new Error('Transaction sign canceled');
    }

    if (!isExpired) {
      const declineExtrinsic = api.voucher.call(voucher.id, { DeclineVoucher: null });
      await sendTransaction(declineExtrinsic, pair, ['VoucherDeclined'], options);
    }

    batchSend(signedTxs, { ...options, onError });
  };

  return { createSession, deleteSession };
}

export { useCreateSession };

export type { Session };
