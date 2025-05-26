import { useProgram as useGearJsProgram } from '@gear-js/react-hooks';

import { ENV } from '@/consts';

import { Program as NftProgram } from './nft';
import { Program as MarketplaceProgram } from './nft_marketplace';

const useMarketplaceProgram = () => {
  const { data: program } = useGearJsProgram({ library: MarketplaceProgram, id: ENV.MARKETPLACE_CONTRACT });

  return program;
};

const useNftProgram = () => {
  const { data: program } = useGearJsProgram({ library: NftProgram, id: ENV.NFT_CONTRACT });

  return program;
};

export { useNftProgram, useMarketplaceProgram };
