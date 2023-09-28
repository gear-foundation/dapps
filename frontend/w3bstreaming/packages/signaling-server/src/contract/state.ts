import { HexString } from '@polkadot/util/types';

import config from '../config';
import { api, meta } from './init';

export async function getState(): Promise<any> {
  const state = await api.programState.read(
    {
      programId: config.programId as HexString,
    },
    meta
  );

  return state.toJSON() as boolean;
}

export async function isUserSubscribed(
  broadcasterId: string,
  watcherId: string
): Promise<boolean> {
  const state = await getState();

  return (
    state.users?.[broadcasterId]?.subscribers?.includes(watcherId) || false
  );
}
