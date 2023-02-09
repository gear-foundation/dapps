import { useApp, useBattle } from 'app/context';
import { useEffect } from 'react';
import { BattlePlayerType, BattleStateResponse } from '../types/battles';
import { useAccount, useApi, useReadFullState, useSendMessage } from '@gear-js/react-hooks';
import { useMetadata } from './use-metadata';
import metaBattle from 'assets/meta/meta-battle.txt';
import { ENV } from '../consts';
import { UnsubscribePromise } from '@polkadot/api/types';
import { UserMessageSent } from '@gear-js/api';

function useReadBattleState<T>() {
  const { metadata } = useMetadata(metaBattle);
  return useReadFullState<T>(ENV.battle, metadata);
}

export function useInitBattleData() {
  const { api } = useApi();
  const { setIsAdmin } = useApp();
  const { account } = useAccount();
  const { setPlayers, setBattleState, setCurrentPlayer } = useBattle();
  const { state } = useReadBattleState<BattleStateResponse>();
  const { metadata } = useMetadata(metaBattle);

  useEffect(() => {
    setBattleState(state);
    if (state && account) {
      setIsAdmin(state.admin === account.decodedAddress);

      const getPlayers = () => {
        const result: BattlePlayerType[] = [];
        state.round.tmgIds.forEach((player, i) => {
          if (state.players[player]) result.push(state.players[player]);
        });
        return result;
      };

      setPlayers(getPlayers());
      setCurrentPlayer(state.round.tmgIds[state.round.moves.length > 0 ? 1 : 0]);

      console.log({ state, players: Object.values(state.players) });
    } else {
      setPlayers([]);
    }
  }, [state, account]);

  useEffect(() => {
    let unsub: UnsubscribePromise | undefined;

    if (metadata && state) {
      const decodedPayload = metadata.getTypeName(12);

      console.log({ decodedPayload });

      // unsub = api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data }: UserMessageSent) => {
      //   const {
      //     message: { payload },
      //   } = data;
      //
      //   const decodedPayload = metadata.createType(8, payload).toHuman();
      //
      //   console.log({ decodedPayload });
      //
      //   // if (tamagotchi && ['WantToSleep', 'PlayWithMe', 'FeedMe'].includes(decodedPayload)) {
      //   //   // const update = getNotificationTypeValue(decodedPayload, tamagotchi);
      //   //   // setNotification((prev) => ({ ...prev, ...update }));
      //   // }
      // });
    }

    return () => {
      if (unsub) unsub.then((unsubCallback) => unsubCallback());
    };
  }, [metadata, state]);
}

export function useBattleMessage() {
  const { metadata } = useMetadata(metaBattle);
  return useSendMessage(ENV.battle, metadata);
}
