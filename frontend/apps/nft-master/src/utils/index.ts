import { isHex } from '@polkadot/util';
import { HexString } from '@polkadot/util/types';
import { Client, cacheExchange, fetchExchange } from 'urql';

import { ENV } from '@/consts';

export const isProgramIdValid = (value: string): value is HexString => isHex(value, 256);

export const urqlClient = new Client({
  url: ENV.EXPLORER_URL || 'https://nft-explorer.vara-network.io/graphql',
  exchanges: [cacheExchange, fetchExchange],
});

export const isMobileDevice = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
  navigator.userAgent,
);
