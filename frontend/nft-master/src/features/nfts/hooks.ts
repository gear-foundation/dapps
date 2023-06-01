import { getProgramMetadata } from '@gear-js/api';
import { useAlert, useApi } from '@gear-js/react-hooks';
import { useEffect, useState } from 'react';
import { useAtom } from 'jotai';
import metaTxt from 'assets/nft_master.meta.txt';
import { useProgramMetadata } from 'hooks';
import { useContractAddress } from '../contract-address';
import { MasterContractState, NFTContractState } from './types';
import { NFTS_ATOM } from './consts';

function useNFTsState() {
  const { api, isApiReady } = useApi();
  const alert = useAlert();

  const masterContractAddress = useContractAddress();
  const masterMetadata = useProgramMetadata(metaTxt);

  const [NFTContracts, setNFTContracts] = useState<MasterContractState['nfts']>();
  const [NFTs, setNFTs] = useAtom(NFTS_ATOM);

  useEffect(() => {
    if (!isApiReady || !masterContractAddress || !masterMetadata) return;

    const programId = masterContractAddress;

    api.programState
      .read({ programId }, masterMetadata)
      .then((codec) => codec.toHuman() as MasterContractState)
      .then(({ nfts }) => setNFTContracts(nfts))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, masterContractAddress, masterMetadata]);

  useEffect(() => {
    if (!NFTContracts) return;

    const promises = NFTContracts.map(([programId, metaRaw]) => {
      const metaHex = `0x${metaRaw}`;
      const metadata = getProgramMetadata(metaHex);

      return api.programState
        .read({ programId }, metadata)
        .then((codec) => codec.toHuman() as NFTContractState)
        .then(({ tokens, collection }) =>
          tokens.map(([id, token]) => ({ ...token, id, programId, collection: collection.name })),
        );
    });

    Promise.all(promises)
      .then((result) => setNFTs(result.flat()))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [NFTContracts]);

  return !!NFTs;
}

function useNFTs() {
  const [NFTs] = useAtom(NFTS_ATOM);

  return NFTs || [];
}

export { useNFTsState, useNFTs };
