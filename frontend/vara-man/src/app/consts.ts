import { HexString } from '@polkadot/util/types'

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
  WALLET: 'wallet',
}

export const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  GAME: import.meta.env.VITE_GAME_ADDRESS,
}

export type IRegisterForm = {
  wallet: HexString | ''
  nickname: string
}
export const initialRegister: IRegisterForm = {
  wallet: '',
  nickname: '',
}
