import { GraphQLClient } from 'graphql-request';

import { ENV } from '@/app/consts';

export { usePokerFactoryProgram, usePokerProgram, usePtsProgram, PokerFactoryProgram } from './sails';

export const graphqlClient = new GraphQLClient(ENV.EXPLORER_URL || '');
