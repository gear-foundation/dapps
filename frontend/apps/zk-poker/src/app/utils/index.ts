import { Client, cacheExchange, fetchExchange } from 'urql';

import { ENV } from '@/app/consts';

export { usePokerFactoryProgram, usePokerProgram, usePtsProgram, PokerFactoryProgram } from './sails';

export const urqlClient = new Client({
  url: ENV.EXPLORER_URL,
  exchanges: [cacheExchange, fetchExchange],
});
