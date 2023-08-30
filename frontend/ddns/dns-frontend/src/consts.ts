import { Hex } from '@gear-js/api';

const ADDRESS = process.env.REACT_APP_NODE_ADDRESS as string;

const CONTRACT_ID = process.env.REACT_APP_PROGRAM_ID as Hex;

export { ADDRESS, CONTRACT_ID };
