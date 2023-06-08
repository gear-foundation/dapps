import { MessagesDispatched, getProgramMetadata } from '@gear-js/api';
import { useAlert, useApi } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { UnsubscribePromise } from '@polkadot/api/types';
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

  const handleStateChange = ({ data }: MessagesDispatched, programId: HexString, onChange: () => void) => {
    const changedIDs = data.stateChanges.toHuman() as HexString[];
    const isAnyChange = changedIDs.some((id) => id === programId);

    if (isAnyChange) onChange();
  };

  const readMasterContractState = () => {
    if (!isApiReady || !masterContractAddress || !masterMetadata) return;

    const programId = masterContractAddress;

    api.programState
      .read({ programId }, masterMetadata)
      .then((codec) => codec.toHuman() as MasterContractState)
      .then(({ nfts }) => setNFTContracts(nfts))
      .catch(({ message }: Error) => alert.error(message));
  };

  useEffect(() => {
    if (!isApiReady || !masterContractAddress || !masterMetadata) return setNFTContracts(undefined);

    readMasterContractState();

    const unsub = api.gearEvents.subscribeToGearEvent('MessagesDispatched', (event) =>
      handleStateChange(event, masterContractAddress, readMasterContractState),
    );

    return () => {
      unsub.then((unsubCallback) => unsubCallback());
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, masterContractAddress, masterMetadata]);

  const readNFTContractsStates = () => {
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
  };

  useEffect(() => {
    if (!NFTContracts) return;

    readNFTContractsStates();

    const unsubs: UnsubscribePromise[] = [];

    // TODO: if state of any contract changes,
    // we reread every single one specified in master contract.
    // need to find the way to optimize
    NFTContracts.forEach(([programId]) => {
      const unsub = api.gearEvents.subscribeToGearEvent('MessagesDispatched', (event) =>
        handleStateChange(event, programId, readNFTContractsStates),
      );

      unsubs.push(unsub);
    });

    return () => {
      unsubs.map((unsub) => unsub.then((unsubCallback) => unsubCallback()));
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [NFTContracts]);

  return masterContractAddress ? !!NFTs : true;
}

function useNFTs() {
  const [NFTs] = useAtom(NFTS_ATOM);

  return NFTs || [];
}

export { useNFTsState, useNFTs };
