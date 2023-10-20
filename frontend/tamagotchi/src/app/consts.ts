import { HexString } from "@polkadot/util/types";

export const LOCAL_STORAGE = {
  ACCOUNT: "account",
};

export const createTamagotchiInitial = {
  programId: "" as HexString,
  programId2: "" as HexString,
  currentStep: 1,
};

export const ENV = {
  store: import.meta.env.VITE_STORE_ADDRESS as HexString,
  balance: import.meta.env.VITE_FT_ADDRESS as HexString,
  battle: import.meta.env.VITE_BATTLE_ADDRESS as HexString,
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
};

export const MULTIPLIER = {
  MILLISECONDS: 1000,
  SECONDS: 60,
  MINUTES: 60,
  HOURS: 24,
};
