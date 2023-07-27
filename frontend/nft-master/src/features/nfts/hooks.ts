import { MessagesDispatched, ProgramMetadata, decodeAddress, getProgramMetadata } from '@gear-js/api';
import { useAccount, useAlert, useApi, useSendMessage } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { UnsubscribePromise } from '@polkadot/api/types';
import { useEffect, useMemo, useState } from 'react';
import { atom, useAtom } from 'jotai';
import metaTxt from 'assets/nft_master.meta.txt';
import { useProgramMetadata } from 'hooks';
import { useSearchParams } from 'react-router-dom';
import { useNodeAddress } from 'features/node-switch';
import { useContractAddress } from '../contract-address';
import { MasterContractState, NFTContractState, TestnetNFTState } from './types';
import { NFTS_ATOM, NFT_CONTRACTS_ATOM, TESTNET_NFT_CONTRACT_ADDRESS } from './consts';

const handleStateChange = ({ data }: MessagesDispatched, programId: HexString, onChange: () => void) => {
  const changedIDs = data.stateChanges.toHuman() as HexString[];
  const isAnyChange = changedIDs.some((id) => id === programId);

  if (isAnyChange) onChange();
};

function useNFTsState() {
  const { api, isApiReady } = useApi();
  const alert = useAlert();

  const { contractAddress: masterContractAddress } = useContractAddress();
  const masterMetadata = useProgramMetadata(metaTxt);

  const [NFTContracts, setNFTContracts] = useAtom(NFT_CONTRACTS_ATOM);
  const [NFTs, setNFTs] = useAtom(NFTS_ATOM);

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
    setNFTContracts(undefined);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [masterContractAddress]);

  useEffect(() => {
    if (!isApiReady || !masterContractAddress || !masterMetadata) return;

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
  const [NFTContracts] = useAtom(NFT_CONTRACTS_ATOM);

  return { nfts: NFTs || [], NFTContracts: NFTContracts || [] };
}

function useNFTSearch() {
  const [searchParams, setSearchParams] = useSearchParams();
  const searchQuery = searchParams.get('query') || '';

  const decodedQueryAddress = useMemo(() => {
    if (!searchQuery) return;

    try {
      return decodeAddress(searchQuery);
    } catch (error) {
      return undefined;
    }
  }, [searchQuery]);

  const resetSearchQuery = () => {
    searchParams.delete('query');

    setSearchParams(searchParams);
  };

  return { searchQuery, decodedQueryAddress, resetSearchQuery };
}

const TESTNET_NFT_ATOM = atom<TestnetNFTState | undefined>(undefined);
const TESTNET_NFT_META = atom<ProgramMetadata | undefined>(undefined);

function useTestnetNFTSetup() {
  const { api, isApiReady } = useApi();
  const alert = useAlert();

  const { isTestnet } = useNodeAddress();

  const { NFTContracts } = useNFTs();

  const [state, setState] = useAtom(TESTNET_NFT_ATOM);
  const [metadata, setMetadata] = useAtom(TESTNET_NFT_META);

  const contract = NFTContracts.find(([address]) => address === TESTNET_NFT_CONTRACT_ADDRESS);
  const metaRaw = contract?.[1];
  const metaHex = metaRaw ? (`0x${metaRaw}` as HexString) : undefined;

  useEffect(() => {
    if (!metaHex) {
      setState(undefined);
      setMetadata(undefined);

      return;
    }

    setMetadata(getProgramMetadata(metaHex));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [metaHex]);

  // TODO: better to obtain state from useNFTs to not read state twice,
  // however current implementation return only list of tokens
  const readState = () => {
    if (!api || !metadata) return;

    api.programState
      .read({ programId: TESTNET_NFT_CONTRACT_ADDRESS }, metadata)
      .then((result) => setState(result.toHuman() as TestnetNFTState))
      .catch(({ message }: Error) => alert.error(message));
  };

  useEffect(() => {
    if (!isTestnet || !isApiReady || !metadata) return;

    readState();

    const unsub = api.gearEvents.subscribeToGearEvent('MessagesDispatched', (event) =>
      handleStateChange(event, TESTNET_NFT_CONTRACT_ADDRESS, readState),
    );

    return () => {
      unsub.then((unsubCallback) => unsubCallback());
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, metadata]);

  return isTestnet ? !!state : true;
}

function useTestnetNFT() {
  const [state] = useAtom(TESTNET_NFT_ATOM);
  const [metadata] = useAtom(TESTNET_NFT_META);

  const { account } = useAccount();
  const sendMessage = useSendMessage(TESTNET_NFT_CONTRACT_ADDRESS, metadata);

  const [isMinting, setIsMitnting] = useState(false);

  const authorizedMinters = state?.constraints.authorizedMinters;
  const isTestnetNFTMintAvailable = !!authorizedMinters?.find((address) => address === account?.decodedAddress);

  const mintTestnetNFT = () => {
    setIsMitnting(true);

    sendMessage({ Mint: null }, { onSuccess: () => setIsMitnting(false), onError: () => setIsMitnting(false) });
  };

  return { mintTestnetNFT, isTestnetNFTMintAvailable, isMinting };
}

function useTestnetAutoLogin() {
  const { isTestnet } = useNodeAddress();

  const { login, accounts, isAccountReady } = useAccount();
  const alert = useAlert();

  const [searchParams, setSearchParams] = useSearchParams();

  useEffect(() => {
    if (!isTestnet || !isAccountReady) return;

    const accountAddress = searchParams.get('account');

    if (accountAddress) {
      const account = accounts.find(({ address }) => address === accountAddress);

      if (account) {
        login(account).then(() => {
          searchParams.delete('account');
          setSearchParams(searchParams);
        });
      } else {
        alert.error(`Account with address ${accountAddress} not found`);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchParams, accounts, isTestnet, isAccountReady]);
}

export { useNFTsState, useNFTs, useNFTSearch, useTestnetNFT, useTestnetAutoLogin, useTestnetNFTSetup };
