import { AnyJson } from '@polkadot/types/types';
import { HexString } from '@polkadot/util/types';

type Session = {
  key: HexString;
  duration: string;
  allowed_actions: AnyJson;
};

type State = {
  SessionForTheAccount: Session | null;
};

export type { State };
