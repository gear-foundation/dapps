import { useAccount, useAlert, useApi } from '@gear-js/react-hooks';
import { MutableRefObject, useRef, useState } from 'react';
import { UnsubscribePromise } from '@polkadot/api/types';
import { Bytes } from '@polkadot/types';
import { ProgramMetadata, UserMessageSent, decodeAddress } from '@gear-js/api';
import { ContractError } from '../types';
import { useSignlessTransactions } from 'gear-ez-transactions';
import { useDnsProgramIds } from '@dapps-frontend/hooks';

export function useWatchMessages<T>(meta: ProgramMetadata) {
  const { programId } = useDnsProgramIds();
  const { api } = useApi();
  const { account } = useAccount();
  const alert = useAlert();
  const signless = useSignlessTransactions();

  const messageSub: MutableRefObject<UnsubscribePromise | null> = useRef(null);
  const [reply, setReply] = useState<T | undefined>();
  const [isOpened, setIsOpened] = useState<boolean>(false);

  const getDecodedPayload = <T>(payload: Bytes) => {
    if (!meta?.types.handle.output) return;
    return meta.createType(meta.types.handle.output, payload).toHuman() as { Ok: T };
  };

  const onChangeState = ({ data: { message } }: UserMessageSent) => {
    console.log('onChangeState message:');
    console.log(message.toHuman());
    const { destination, source, payload } = message;

    const signlessPairAddress = signless.pair?.address;
    const ownerAddress = signlessPairAddress ? decodeAddress(signlessPairAddress) : account?.decodedAddress;
    const isOwner = destination.toHex() === ownerAddress;

    const isGame = source.toHex() === programId;

    if (isOwner && isGame) {
      try {
        const reply = getDecodedPayload<T>(payload);
        console.log('inside update: ', { reply });

        const { Ok } = reply || {};

        setReply(Ok);
      } catch (e) {
        console.log(e);
        alert.error((e as ContractError).message);
      }
    }
  };

  const subscribe = () => {
    if (!api || messageSub.current) return;
    console.log('subscribed!');

    setIsOpened(true);
    messageSub.current = api.gearEvents.subscribeToGearEvent('UserMessageSent', onChangeState);
  };

  const unsubscribe = () => {
    console.log('unsubscribed  :)');
    messageSub.current?.then((unsubCb) => {
      messageSub.current = null;
      unsubCb();
      setIsOpened(false);
      setReply(undefined);
    });
  };

  return {
    subscribe,
    unsubscribe,
    reply,
    isOpened,
  };
}
