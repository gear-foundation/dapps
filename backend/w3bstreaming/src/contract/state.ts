import { HexString } from '@polkadot/util/types';

import config from '../config';
import { api, meta } from './init';
import { UsersState } from './types';

export async function getStateUsers(): Promise<UsersState> {
  const state = await api.programState.read(
    {
      programId: config.programId as HexString,
      payload: {
        Users: null,
      },
    },
    meta
  );

  return state.toHuman() as unknown as UsersState;
}

export async function isUserSubscribed(
  broadcasterId: string,
  watcherId: string
): Promise<boolean> {
  const state = await getStateUsers();

  return (
    state.Users?.find(
      user => user[0] === broadcasterId
    )?.[1]?.subscribers?.includes(watcherId) || false
  );
}
