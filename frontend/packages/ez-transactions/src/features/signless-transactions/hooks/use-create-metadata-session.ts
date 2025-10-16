import { HexString, ProgramMetadata, decodeAddress } from '@gear-js/api';
import { Account, useAccount, useApi } from '@gear-js/react-hooks';
import { KeyringPair } from '@polkadot/keyring/types';
import { AnyJson } from '@polkadot/types/types';

import { sendTransaction, signHex } from '../utils';

import { CreeateSessionOptions, Options, Session, useCreateBaseSession } from './use-create-base-session';

function useCreateMetadataSession(
  programId: HexString,
  metadata: ProgramMetadata | undefined,
  createSignatureType?: (_metadata: ProgramMetadata, payloadToSig: Session) => `0x${string}`,
) {
  const { api, isApiReady } = useApi();
  const { account } = useAccount();

  const { signAndSendCreateSession, signAndSendDeleteSession, updateVoucherBalance, onError } =
    useCreateBaseSession(programId);

  const getMessageExtrinsic = (payload: AnyJson) => {
    if (!isApiReady) throw new Error('API is not initialized');
    if (!metadata) throw new Error('Metadata not found');

    const destination = programId;
    const gasLimit = 250000000000; // TODO: replace with calculation after release fix

    return api.message.send({ destination, payload, gasLimit }, metadata);
  };

  const getAccountSignature = async (_metadata: ProgramMetadata, _account: Account, payloadToSign: Session) => {
    if (_metadata.types?.others?.output === null) {
      throw new Error(`Metadata type doesn't exist`);
    }

    console.log('metadata.types?.others?.output');
    console.log(_metadata.types?.others?.output);

    const hexToSign = createSignatureType
      ? createSignatureType(_metadata, payloadToSign)
      : _metadata.createType(_metadata.types.others.output, payloadToSign).toHex();

    return signHex(_account, hexToSign);
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
    return signAndSendCreateSession(messageExtrinsic, session, voucherValue, options, shouldIssueVoucher);
  };

  const deleteSession = async (key: HexString, pair: KeyringPair, options: Options) => {
    if (!account) throw new Error('Account not found');
    if (!metadata) throw new Error('Metadata not found');

    const messageExtrinsic = getMessageExtrinsic({
      DeleteSessionFromAccount: null,
    });

    return signAndSendDeleteSession(messageExtrinsic, key, pair, options);
  };

  return { createSession, deleteSession, updateVoucherBalance };
}

export { useCreateMetadataSession };
