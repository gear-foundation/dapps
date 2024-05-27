import { useEffect, useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAccount, useApi, useSendMessage } from '@gear-js/react-hooks';
import { useProgramMetadata } from 'app/hooks/api';
import { useBattle } from '../context';
import { BATTLE_ADDRESS } from '../consts';
import meta from '../assets/meta/battle.meta.txt';
import type { HexString, UserMessageSent } from '@gear-js/api';
import type { UnsubscribePromise } from '@polkadot/api/types';
import type { BattleStatePlayer } from '../types/battles';
import type { BattleCurrentStateVariants, RoundDamageType } from '../types/battles';
import { useFullState } from './use-full-state';

const programId = BATTLE_ADDRESS;

export function useInitBattleData() {
  const { api } = useApi();
  const navigate = useNavigate();
  const { account } = useAccount();
  const {
    roundDamage,
    currentPairIdx,
    setRivals,
    setBattle,
    setCurrentPlayer,
    setCurrentPairIdx,
    setRoundDamage,
    setPlayers,
    setIsAdmin,
  } = useBattle();

  const state = useFullState(programId, meta);

  const prevBattleState = useRef<BattleCurrentStateVariants | undefined>();
  const metadata = useProgramMetadata(meta);

  useEffect(() => {
    if (window) {
      (window as any).BattleAddress = process.env.REACT_APP_CONTRACT_ADDRESS as HexString;
    }
  }, []);

  useEffect(() => {
    setBattle(state);
    if (state && account) {
      const activePair = Object.values(state.pairs)[currentPairIdx];
      // console.log({ state });
      setIsAdmin(state.admins.includes(account.decodedAddress));

      const getCurrentQueue = () => {
        const queue: BattleStatePlayer[] = [];
        state.currentPlayers.forEach((p) => queue.push(state.players[p]));
        return queue;
      };
      const players = getCurrentQueue();
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
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state, account, currentPairIdx]);

  useEffect(() => {
    let unsub: UnsubscribePromise | undefined;

    if (metadata && state) {
      unsub = api?.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data }: UserMessageSent) => {
        const {
          message: { payload, details },
        } = data;

        if (details.isSome && !details.unwrap().to.eq(0)) {
          return;
        }

        if (!metadata.types.handle.output) {
          return;
        }

        const decodedPayload = metadata.createType(metadata.types.handle.output, payload).toJSON();

        if (
          decodedPayload &&
          typeof decodedPayload === 'object' &&
          'roundResult' in decodedPayload
        ) {
          const notification = Object.values(decodedPayload)[0] as RoundDamageType;

          if (currentPairIdx === +notification[0]) {
            setRoundDamage(notification);
          }
        }
      });
    }

    return () => {
      if (unsub) unsub.then((unsubCallback) => unsubCallback());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [metadata, state, currentPairIdx]);

  // track state updates
  useEffect(() => {
    if (state) {
      if (prevBattleState.current === 'WaitNextRound' && state.state === 'GameIsOn') setCurrentPairIdx(0);

      if (!state.pairs[currentPairIdx]) setCurrentPairIdx(0);

      if (prevBattleState.current === 'GameIsOver' && state.state === 'Registration') navigate('/');

      if (prevBattleState.current !== state.state) prevBattleState.current = state.state;
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [navigate, state]);

  // track damage updates
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
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [currentPairIdx, roundDamage, state]);
}

export function useBattleMessage() {
  const metadata = useProgramMetadata(meta);

  return useSendMessage(programId, metadata);
}
