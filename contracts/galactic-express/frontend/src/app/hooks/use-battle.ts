import { useApp, useLounch } from 'app/context';
import { useEffect } from 'react';
import type { BattlePlayerType, BattleStateResponse, LouncheStateResponse } from 'app/types/battles';
import { useAccount, useApi, useReadFullState } from '@gear-js/react-hooks';
import { useMetadata } from './use-metadata';
import metaBattle from 'assets/meta/meta.txt';
import { ENV } from 'app/consts';
import type { UnsubscribePromise } from '@polkadot/api/types';
import type { UserMessageSent } from '@gear-js/api';
import { useSendMessage } from './useSendMessage';
import { RoundDamageType } from 'app/types/battles';

function useReadLouncheState<T>() {
  const { metadata } = useMetadata(metaBattle);
  return useReadFullState<T>(ENV.contract, metadata);
}

export function useInitLouncheData() {

  const { setIsAdmin, setIsDataReady } = useApp();
  const { setLaunch, setSessionIsOver } = useLounch();
  const { account } = useAccount();
  const { state } = useReadLouncheState<LouncheStateResponse>();

  useEffect(() => {
    if (state && account) {
      setIsAdmin(state.owner === account.decodedAddress);
      setLaunch(state);

      if (state.state === 'SessionIsOver') {
        setSessionIsOver(true);
      } else {
        setSessionIsOver(false);
      }
    }

  }, [state, account]);
}

// export function useInitBattleData() {
//   const { api } = useApi();
//   const { setIsAdmin, setIsDataReady } = useApp();
//   const { account } = useAccount();
//   const { roundDamage, setRivals, setBattle, setCurrentPlayer, setRoundDamage, setPlayers } = useBattle();
//   const { state } = useReadBattleState<BattleStateResponse>();
//   const { metadata } = useMetadata(metaBattle);

//   useEffect(() => {
//     setBattle(state);
//     if (state && account) {
//       setIsAdmin(state.admin === account.decodedAddress);

//       const getPlayers = () => {
//         const result: BattlePlayerTying[] = [];
//         state.round.tmgIds.forEach((player, i) => {
//           if (state.players[player]) result.push(state.players[player]);
//         });
//         return result;
//       };

//       setPlayers(Object.values(state.players));
//       setRivals(getPlayers());
//       setCurrentPlayer(state.round.tmgIds[state.round.moves.length > 0 ? 1 : 0]);
//     } else {
//       setPlayers([]);
//       setRivals([]);
//     }
//   }, [state, account]);

//   useEffect(() => {
//     let unsub: UnsubscribePromise | undefined;

//     if (metadata && state) {
//       unsub = api.gearEvents.subscribeToGearEvent('UserMessageSent', ({ data }: UserMessageSent) => {
//         const {
//           message: { payload, details },
//         } = data;

//         if (details.isSome && details.unwrap().isReply && !details.unwrap().asReply.statusCode.eq(0)) {
//           console.log(payload.toHuman());
//         } else {
//           const decodedPayload = metadata.createType(5, payload).toJSON();

//           if (
//             decodedPayload &&
//             typeof decodedPayload === 'object' &&
//             Object.keys(decodedPayload).includes('roundResult')
//           ) {
//             console.log({ decodedPayload });
//             setRoundDamage(Object.values(decodedPayload)[0] as RoundDamageType);
//           }
//         }
//       });
//     }

//     return () => {
//       if (unsub) unsub.then((unsubCallback) => unsubCallback());
//     };
//   }, [metadata, state]);

//   useEffect(() => {
//     if (state) {
//       if (state.round.steps && !state.round.moves.length) {
//         // console.log('show damage');
//       } else {
//         if (roundDamage) {
//           // console.log('hide damage');
//           setRoundDamage(undefined);
//         }
//       }
//     }
//   }, [roundDamage, state]);
// }

export function useBattleMessage() {
  const { metadata } = useMetadata(metaBattle);
  return useSendMessage(ENV.contract, metadata);
}
