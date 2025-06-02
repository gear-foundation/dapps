export const ROUTES = {
  HOME: '/',
  LOGIN: '/login',
  NOTFOUND: '*',
};

export const LOCAL_STORAGE = {
  ACCOUNT: 'account',
};

export const ENV = {
  DNS_API_URL: import.meta.env.VITE_DNS_API_URL as string,
  DNS_NAME: import.meta.env.VITE_DNS_NAME as string,
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
};

export const playerNames = ['Rojo', 'Oscuro', 'Naranja', 'Amarillo', 'Gris', 'Verde', 'Azul', 'Morado'];
