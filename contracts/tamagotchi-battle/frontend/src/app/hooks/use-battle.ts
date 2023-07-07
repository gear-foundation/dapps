import { useApp, useBattle } from "app/context";
import { useEffect, useRef } from "react";
import type { BattleStatePlayer, BattleStateResponse } from "app/types/battles";
import { useAccount, useApi, useSendMessage } from "@gear-js/react-hooks";
import { useProgramMetadata, useReadState } from "./api";
import meta from "assets/meta/battle.meta.txt";
import { ENV } from "app/consts";
import type { UnsubscribePromise } from "@polkadot/api/types";
import type { UserMessageSent } from "@gear-js/api";
import { BattleCurrentStateVariants, RoundDamageType } from "app/types/battles";
import { useNavigate } from "react-router-dom";

const programId = ENV.battle;

export function useInitBattleData() {
  const { api } = useApi();
  // const alert = useAlert();
  const navigate = useNavigate();
  const { setIsAdmin } = useApp();
  const { account } = useAccount();
  const {
    roundDamage,
    currentPairIdx,
    setRivals,
    setBattle,
    setCurrentPlayer,
    setCurrentPairIdx,
    setRoundDamage,
    setPlayers
  } = useBattle();
  const { state } = useReadState<BattleStateResponse>({ programId, meta });
  const prevBattleState = useRef<BattleCurrentStateVariants | undefined>();
  const metadata = useProgramMetadata(meta);

  useEffect(() => {
    if (window) {
      (window as any).BattleAddress = process.env.REACT_APP_BATTLE_ADDRESS;
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
      unsub = api.gearEvents.subscribeToGearEvent("UserMessageSent", ({ data }: UserMessageSent) => {
        const {
          message: { payload, details }
        } = data;

        if (details.isSome && !details.unwrap().to.eq(0)) {
          // console.log(payload.toHuman());
          // alert.error(`${payload.toHuman()}`, { title: 'Error during program execution' });
        } else {
          if (metadata.types.handle.output) {
            const decodedPayload = metadata.createType(metadata.types.handle.output, payload).toJSON();
            if (
              decodedPayload &&
              typeof decodedPayload === "object" &&
              Object.keys(decodedPayload).includes("roundResult")
            ) {
              const notification = Object.values(decodedPayload)[0] as RoundDamageType;

              if (currentPairIdx === notification[0]) {
                // console.log({ decodedPayload });
                setRoundDamage(notification);
              }
            }
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
      if (prevBattleState.current === "WaitNextRound" && state.state === "GameIsOn") setCurrentPairIdx(0);

      if (prevBattleState.current === "GameIsOver" && state.state === "Registration") navigate("/");

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
  return useSendMessage(programId, metadata, { isMaxGasLimit: true });
}
