import { atom } from 'jotai'

const AUTH_API_ADDRESS = import.meta.env.VITE_AUTH_API_ADDRESS as string

const AUTH_MESSAGE = 'VARA'

const AUTH_TOKEN_LOCAL_STORAGE_KEY = 'authToken'

const AUTH_TOKEN_ATOM = atom<string | null>(
  localStorage.getItem(AUTH_TOKEN_LOCAL_STORAGE_KEY)
)

export {
  AUTH_API_ADDRESS,
  AUTH_MESSAGE,
  AUTH_TOKEN_LOCAL_STORAGE_KEY,
  AUTH_TOKEN_ATOM,
}
