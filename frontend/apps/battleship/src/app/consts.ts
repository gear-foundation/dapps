export const ACCOUNT_ID_LOCAL_STORAGE_KEY = 'account';

export const ADDRESS = {
  NODE: import.meta.env.VITE_NODE_ADDRESS,
  GASLESS_BACKEND: import.meta.env.VITE_GASLESS_BACKEND_ADDRESS,
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL,
  DNS_NAME: import.meta.env.VITE_DNS_NAME,
};

export const ROUTES = {
  HOME: '/',
  GAME: '/game',
  NOTFOUND: '*',
};

export const SIGNLESS_ALLOWED_ACTIONS = ['StartGame', 'Turn'];
