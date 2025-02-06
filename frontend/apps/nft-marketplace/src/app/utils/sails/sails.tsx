import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';

import { ADDRESS } from '@/consts';

import { Program as NftProgram } from './nft';
import { Program as MarketplaceProgram } from './nft_marketplace';

const useMarketplaceProgram = () => {
  const { data: program } = useGearJsProgram({ library: MarketplaceProgram, id: ADDRESS.MARKETPLACE_CONTRACT });

  return program;
};

const useNftProgram = () => {
  const { data: program } = useGearJsProgram({ library: NftProgram, id: ADDRESS.NFT_CONTRACT });

  return program;
};

export { useNftProgram, useMarketplaceProgram };
