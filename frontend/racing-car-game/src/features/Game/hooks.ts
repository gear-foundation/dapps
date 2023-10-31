import { useCallback, useEffect, useRef, useState } from 'react';
import { useAtom, useSetAtom } from 'jotai';
import { UserMessageSent } from '@gear-js/api';
import { useAccount, useAlert, useApi, useSendMessage } from '@gear-js/react-hooks';
import { Bytes } from '@polkadot/types';
import { UnsubscribePromise } from '@polkadot/api/types';
import isEqual from 'lodash.isequal';
import { ADDRESS } from '@/consts';
import { useProgramMetadata } from '@/hooks';
import metaTxt from '@/assets/meta/meta.txt';
import { ContractError, DecodedReply, GameState } from '@/types';
import { CURRENT_SENT_MESSAGE_ID_ATOM, IS_STATE_READ_ATOM, REPLY_DATA_ATOM } from './atoms';
import { logger } from '@/utils';
import { IS_SUBSCRIBED_ATOM } from '@/atoms';

function usePlayerMoveMessage() {
  const meta = useProgramMetadata(metaTxt);

  return useSendMessage(ADDRESS.CONTRACT, meta, {
    isMaxGasLimit: true,
    disableAlerts: true,
  });
}

function useStartGameMessage() {
  const meta = useProgramMetadata(metaTxt);

  const message = useSendMessage(ADDRESS.CONTRACT, meta, {
    isMaxGasLimit: true,
  });

  return { meta, message };
}

function useSubscribeSentMessage() {
  const { api } = useApi();
  const alert = useAlert();
  const messageSubscription: React.MutableRefObject<UnsubscribePromise | null> = useRef(null);
  const meta = useProgramMetadata(metaTxt);
  const { account } = useAccount();
  const [replyData, setReplyData] = useAtom(REPLY_DATA_ATOM);
  const [currentSentMessageId, setCurrentSentMessageId] = useAtom(CURRENT_SENT_MESSAGE_ID_ATOM);
  const [isSubscribed, setIsSubscribed] = useAtom(IS_SUBSCRIBED_ATOM);

  const handleUnsubscribeFromEvent = (onSuccess?: () => void) => {
    if (messageSubscription.current) {
      messageSubscription.current?.then((unsubCallback) => {
        unsubCallback();
        logger('UNsubscribed from reply');
        onSuccess?.();
      });
    }
  };

  const getDecodedPayload = (payload: Bytes) => {
    if (meta?.types.others.output) {
      return meta.createType(meta?.types.others.output, payload).toHuman();
    }
  };

  const getDecodedReply = (payload: Bytes): DecodedReply => {
    const decodedPayload = getDecodedPayload(payload);

    return decodedPayload as DecodedReply;
  };

  const clearReplyData = () => {
    setReplyData(null);
  };

  const handleChangeState = useCallback(
    ({ data: _data }: UserMessageSent) => {
      const { message } = _data;
      const { destination, source, payload } = message;
      const isOwner = destination.toHex() === account?.decodedAddress;
      const isCurrentProgram = source.toHex() === ADDRESS.CONTRACT;
      // const isNeededMessageId = id.toHex() === currentSentMessageId;

      if (isOwner && isCurrentProgram) {
        logger('new message:');
        logger(message.toHuman());
        logger('trying to decode....:');
        try {
          const reply = getDecodedReply(payload);
          logger('DECODED message successfully');
          logger('new reply HAS COME:');
          logger(reply);

          if (reply && reply.cars.length && !isEqual(reply?.cars, replyData?.cars)) {
            logger('prev reply state:');
            logger(replyData);
            logger('new reply UPDATED and going to state:');
            logger(reply);
            setReplyData(reply);
            setCurrentSentMessageId(null);
            // handleUnsubscribeFromEvent();
          }
        } catch (e) {
          logger(e);
          alert.error((e as ContractError).message);
        }
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [replyData, meta],
  );

  const handleSubscribeToEvent = () => {
    if (api) {
      messageSubscription.current = api.gearEvents.subscribeToGearEvent('UserMessageSent', handleChangeState);
      logger('subscribed on reply');
    }
  };

  useEffect(() => {
    if (meta && !isSubscribed) {
      handleSubscribeToEvent();
      setIsSubscribed(true);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [meta, isSubscribed]);

  return { handleSubscribeToEvent, handleUnsubscribeFromEvent, replyData, clearReplyData };
}

function useGameState() {
  const { api } = useApi();
  const programId = ADDRESS.CONTRACT;
  const meta = useProgramMetadata(metaTxt);
  const [gameData, setGameData] = useState<GameState | null>(null);
  const setReplyData = useSetAtom(REPLY_DATA_ATOM);
  const [isStateRead, setIsStateRead] = useAtom(IS_STATE_READ_ATOM);
  const { account, isAccountReady } = useAccount();

  const handleReadState = useCallback(async () => {
    if (api && meta && programId && isAccountReady && !isStateRead) {
      try {
        const res = await api.programState.read(
          {
            programId: ADDRESS.CONTRACT,
            payload: {
              Game: {
                account_id: account?.decodedAddress,
              },
            },
          },
          meta,
        );
        logger('state init');
        const state = (await res.toHuman()) as any;
        setGameData(state.Game);
        setIsStateRead(true);
      } catch (err) {
        logger(err);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [meta, api, programId, isAccountReady, isStateRead, account?.decodedAddress]);

  useEffect(() => {
    if (account?.decodedAddress) {
      setIsStateRead(false);
      setReplyData(null);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account?.decodedAddress]);

  useEffect(() => {
    handleReadState();
  }, [handleReadState]);

  return { state: { Game: gameData || null }, isStateRead, setIsStateRead };
}

export { usePlayerMoveMessage, useStartGameMessage, useGameState, useSubscribeSentMessage };
