import { getProgramMetadata } from '@gear-js/api';
import { useApi, useReadFullState } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { useEffect, useState } from 'react';
import metaTxt from 'assets/nft_master.meta.txt';
import { useProgramMetadata } from 'hooks';
import { useContractAddress } from '../contract-address';
import { MasterContractState, NFTContractState } from './types';

function useNFTsState() {
  const { api, isApiReady } = useApi();

  const contractAddress = useContractAddress();
  const masterMetadata = useProgramMetadata(metaTxt);

  const { state } = useReadFullState<MasterContractState>(contractAddress, masterMetadata);
  const nftContracts = state?.nfts;

  const [nftStates, setNftStates] = useState<(NFTContractState & { programId: HexString })[]>();

  useEffect(() => {
    if (!isApiReady || !nftContracts) return;

    const nftStatePromises = nftContracts.map(([programId, metaRaw]) => {
      const metaHex = `0x${metaRaw}`;
      const metadata = getProgramMetadata(metaHex);

      return api.programState
        .read({ programId }, metadata)
        .then((codec) => codec.toHuman() as NFTContractState)
        .then((nftState) => ({ ...nftState, programId }));
    });

    Promise.all(nftStatePromises).then((result) => setNftStates(result));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, nftContracts]);

  return nftStates;
}

export { useNFTsState };
