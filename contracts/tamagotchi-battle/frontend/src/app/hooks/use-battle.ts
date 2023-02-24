import { useApp, useBattle } from 'app/context';
import { useEffect } from 'react';
import type { BattleStatePlayer, BattleStateResponse } from 'app/types/battles';
import { useAccount, useApi, useReadFullState } from '@gear-js/react-hooks';
import { useMetadata } from './use-metadata';
import metaBattle from 'assets/meta/meta-battle.txt';
import { ENV } from 'app/consts';
import type { UnsubscribePromise } from '@polkadot/api/types';
import type { UserMessageSent } from '@gear-js/api';
import { useSendMessage } from './use-send-message';
import { RoundDamageType } from 'app/types/battles';

function useReadBattleState<T>() {
  const { metadata } = useMetadata(metaBattle);
  return useReadFullState<T>(ENV.battle, metadata);
}

export function useInitBattleData() {
  const { api } = useApi();
  const { setIsAdmin } = useApp();
  const { account } = useAccount();
  const { roundDamage, currentPairIdx, setRivals, setBattle, setCurrentPlayer, setRoundDamage, setPlayers } =
    useBattle();
  const { state } = useReadBattleState<BattleStateResponse>();
  const { metadata } = useMetadata(metaBattle);

  useEffect(() => {
    setBattle(state);
    if (state && account) {
      const activePair = Object.values(state.pairs)[currentPairIdx];
      console.log({ state });
      setIsAdmin(state.admin === account.decodedAddress);

      const players = Object.values(state.players);
      players && setPlayers(players);

      if (activePair) {
        const getRivals = () => {
          const result: BattleStatePlayer[] = [];
          activePair.tmgIds.forEach((player) => {
            if (state.players[player]) result.push(state.players[player]);
          });
          // console.log({ rivals: result });
          return result;
        };

        setRivals(getRivals());
        setCurrentPlayer(activePair.tmgIds[activePair.moves.length > 0 ? 1 : 0]);
      }
    } else {
      setIsAdmin(false);
      setPlayers([]);
      setRivals([]);
      setCurrentPlayer(undefined);
    }
  }, [state, account, currentPairIdx]);

  useEffect(() => {
    let unsub: UnsubscribePromise | undefined;

    if (metadata && state) {
      unsub = api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data }: UserMessageSent) => {
        const {
          message: { payload, details },
        } = data;

        if (details.isSome && details.unwrap().isReply && !details.unwrap().asReply.statusCode.eq(0)) {
          console.log(payload.toHuman());
        } else {
          const decodedPayload = metadata.createType(5, payload).toJSON();

          if (
            decodedPayload &&
            typeof decodedPayload === 'object' &&
            Object.keys(decodedPayload).includes('roundResult')
          ) {
            const notification = Object.values(decodedPayload)[0] as RoundDamageType;

            if (currentPairIdx === notification[0]) {
              console.log({ decodedPayload });
              setRoundDamage(notification);
            }
          }
        }
      });
    }

    return () => {
      if (unsub) unsub.then((unsubCallback) => unsubCallback());
    };
  }, [metadata, state, currentPairIdx]);

  useEffect(() => {
    if (state) {
      const activePair = Object.values(state.pairs)[currentPairIdx];
      if (activePair && activePair.rounds && !activePair.moves.length) {
        // console.log('show damage');
      } else {
        if (roundDamage) {
          // console.log('hide damage');
          setRoundDamage(undefined);
        }
      }
    }
  }, [currentPairIdx, roundDamage, state]);
}

export function useBattleMessage() {
  const { metadata } = useMetadata(metaBattle);
  return useSendMessage(ENV.battle, metadata);
}
