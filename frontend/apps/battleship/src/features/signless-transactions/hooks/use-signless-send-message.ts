import { AnyJson } from '@polkadot/types/types';
import { HexString, ProgramMetadata } from '@gear-js/api';
import {
  SendMessageOptions,
  UseSendMessageOptions,
  useAccount,
  useSendMessage,
  useSendMessageHandler,
} from '@gear-js/react-hooks';

import { useSignlessTransactions } from '../context';

type SendSignlessMessageOptions = Omit<SendMessageOptions, 'payload'> & { payload: Record<string, AnyJson> };

function useSignlessSendMessage(
  destination: HexString,
  metadata: ProgramMetadata | undefined,
  options?: UseSendMessageOptions,
) {
  const { account } = useAccount();
  const { pair } = useSignlessTransactions();
  const sendMessage = useSendMessage(destination, metadata, { ...options, pair });

  const sendSignlessMessage = (args: SendSignlessMessageOptions) => {
    const sessionForAccount = pair ? account?.decodedAddress : null;
    const payload = { ...args.payload, sessionForAccount };

    const withVoucher = !!pair;

    sendMessage({ ...args, payload, withVoucher });
  };

  return sendSignlessMessage;
}

function useSignlessSendMessageHandler(
  destination: HexString,
  metadata: ProgramMetadata | undefined,
  options?: UseSendMessageOptions & { isMaxGasLimit?: boolean },
) {
  const { account } = useAccount();
  const { pair } = useSignlessTransactions();
  const sendMessage = useSendMessageHandler(destination, metadata, { ...options, pair });

  const sendSignlessMessage = (args: Omit<SendSignlessMessageOptions, 'gasLimit'>) => {
    const sessionForAccount = pair ? account?.decodedAddress : null;
    const payload = { ...args.payload, sessionForAccount };
    const withVoucher = !!pair;

    sendMessage({ ...args, payload, withVoucher });
  };

  return sendSignlessMessage;
}

export { useSignlessSendMessage, useSignlessSendMessageHandler };
export type { SendSignlessMessageOptions };
