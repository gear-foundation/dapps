import { createContext, Dispatch, ReactNode, SetStateAction, useState } from 'react';
import { BattlePlayerType, SessionData, Participant, SessionStatus, LouncheStateResponse } from 'app/types/battles';
import { HexString } from '@polkadot/util/types';

type Program = {
  launch?: LouncheStateResponse;
  setLaunch: Dispatch<SetStateAction<LouncheStateResponse | undefined>>;
  status: SessionStatus;
  setStatus: Dispatch<SetStateAction<SessionStatus>>
  sessionIsOver: boolean;
  setSessionIsOver: Dispatch<SetStateAction<boolean>>;
  players: Participant;
  setPlayers: Dispatch<SetStateAction<Participant>>;
  sessionData: SessionData;
  setSessionData: Dispatch<SetStateAction<SessionData>>;
};

const useProgram = (): Program => {
  const [launch, setLaunch] = useState<LouncheStateResponse>();
  const [players, setPlayers] = useState<Participant>({});
  const [status, setStatus] = useState<SessionStatus>(SessionStatus.SESSION_IS_OVER);
  const [sessionData, setSessionData] = useState<SessionData | any>();
  const [sessionIsOver, setSessionIsOver] = useState<boolean>(false);

  return {
    launch,
    setLaunch,
    players,
    setPlayers,
    status,
    setStatus,
    sessionData,
    setSessionData,
    sessionIsOver,
    setSessionIsOver
  };
};

export const BattleCtx = createContext({} as Program);

export function BattleProvider({ children }: { children: ReactNode }) {
  const { Provider } = BattleCtx;
  return <Provider value={useProgram()}>{children}</Provider>;
}
