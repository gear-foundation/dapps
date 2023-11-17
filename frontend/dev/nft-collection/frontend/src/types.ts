import { HexString } from '@polkadot/util/types';
import { FunctionComponent, SVGProps } from 'react';

export type Entries<T> = {
  [K in keyof T]: [K, T[K]];
}[keyof T][];

export type SVGComponent = FunctionComponent<
  SVGProps<SVGSVGElement> & {
    title?: string | undefined;
  }
>;

export type Handler = (event: Event) => void;

type CollectionPrefs = {
  address: HexString;
  timeCreation: string;
  collectionId: string;
};

export type OwnerToCollection = [HexString, CollectionPrefs][];

export interface ProgramFactoryState {
  collectionCodeId: string;
  ownerToAddress: OwnerToCollection;
  sft: null;
  txId: number;
}

export type Token = {
  owner: string;
  medium: string;
  timeMinted: string;
};

export interface CollectionState {
  availableMedia: string[];
  tokens: [string, Token][];
  owner: string;
  transactions: [];
  collection: {
    name: string;
    description: string;
  };
  nonce: string;
  constraints: {
    admins: [string];
    authorizedMinters: [];
    verifiedContracts: [];
  };
  tokensMetadata: [];
}

export interface ProgramStateRes {
  state?: ProgramFactoryState;
  isStateRead: Boolean;
  error: string;
}
