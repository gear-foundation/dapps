import { HexString } from "@polkadot/util/types";

import { program } from "./init";
import { ProgramState } from "./lib";

export async function getStateUsers(): Promise<ProgramState["users"]> {
  const { users } = await program.w3Bstreaming.getState();
  return users;
}

export async function isUserSubscribed(
  broadcasterId: string,
  watcherId: HexString
): Promise<boolean> {
  const users = await getStateUsers();

  return !!users
    ?.find((user) => user[0] === broadcasterId)?.[1]
    ?.subscribers?.includes(watcherId);
}
