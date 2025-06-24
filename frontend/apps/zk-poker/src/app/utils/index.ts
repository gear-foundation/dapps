import { Client, cacheExchange, fetchExchange } from 'urql';

import { ADDRESS } from '@/app/consts';

export { usePokerFactoryProgram, usePokerProgram, usePtsProgram } from './sails';

export const urqlClient = new Client({
  url: ADDRESS.EXPLORER_URL,
  exchanges: [cacheExchange, fetchExchange],
});
