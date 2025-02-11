import { createContext } from 'react';

import { DEFAULT_SIGNLESS_CONTEXT } from './consts';
import { SignlessContext } from './types';

const SignlessTransactionsContext = createContext<SignlessContext>(DEFAULT_SIGNLESS_CONTEXT);

export { SignlessTransactionsContext };
