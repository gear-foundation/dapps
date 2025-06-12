export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ADDRESS = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  GASLESS_BACKEND: import.meta.env.VITE_GASLESS_BACKEND_ADDRESS,
  ZK_POKER_BACKEND: import.meta.env.VITE_ZK_POKER_BACKEND_ADDRESS as string,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL as string,
  DNS_NAME: import.meta.env.VITE_DNS_NAME as string,
};
console.log('🚀 ~ ADDRESS:', ADDRESS);

export const ROUTES = {
  HOME: '/',
  LOGIN: '/login',
  GAME: '/game/:gameId',
  ONBOARDING: '/onboarding',
  CREATE_GAME: '/create-game',
  ROOMS: '/rooms',
  NOTFOUND: '*',
  COMBINATIONS: '/combinations',
};

export const SIGNLESS_ALLOWED_ACTIONS = ['playSingleGame', 'playMultipleGame'];
