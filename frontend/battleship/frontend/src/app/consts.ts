import { HexString } from '@gear-js/api'

export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account'

export const ADDRESS = {
  NAME: import.meta.env.VITE_NAME_ADDRESS,
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  BACK: import.meta.env.VITE_BACK_ADDRESS,
  GAME: import.meta.env.VITE_GAME_ADDRESS as HexString,
}

export const ROUTES = {
  HOME: '/',
  GAME: '/game',
  NOTFOUND: '*',
}
