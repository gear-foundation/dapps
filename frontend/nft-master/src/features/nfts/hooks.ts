import { MessagesDispatched, decodeAddress, getProgramMetadata } from '@gear-js/api';
import { useAccount, useAlert, useApi, useReadFullState, useSendMessage } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { UnsubscribePromise } from '@polkadot/api/types';
import { useEffect, useMemo } from 'react';
import { useAtom } from 'jotai';
import metaTxt from 'assets/nft_master.meta.txt';
import { useProgramMetadata } from 'hooks';
import { useSearchParams } from 'react-router-dom';
import { useNodeAddress } from 'features/node-switch';
import { useContractAddress } from '../contract-address';
import { MasterContractState, NFTContractState, TestnetNFTState } from './types';
import { NFTS_ATOM, NFT_CONTRACTS_ATOM, TESTNET_NFT_CONTRACT_ADDRESS } from './consts';

function useNFTsState() {
  const { api, isApiReady } = useApi();
  const alert = useAlert();

  const { contractAddress: masterContractAddress } = useContractAddress();
  const masterMetadata = useProgramMetadata(metaTxt);

  const [NFTContracts, setNFTContracts] = useAtom(NFT_CONTRACTS_ATOM);
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

function useTestnetNFT(NFTContracts: [HexString, string][]) {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const contract = NFTContracts.find(([address]) => address === TESTNET_NFT_CONTRACT_ADDRESS);

  const metaRaw = contract?.[1];
  const metaHex = metaRaw ? (`0x${metaRaw}` as HexString) : undefined;

  const metadata = useMemo(() => (metaHex ? getProgramMetadata(metaHex) : undefined), [metaHex]);

  // TODO: better to obtain state from useNFTs to not read state twice,
  // however current implementation return only list of tokens
  const { state } = useReadFullState<TestnetNFTState>(TESTNET_NFT_CONTRACT_ADDRESS, metadata);
  const authorizedMinters = state?.constraints.authorizedMinters;
  const isTestnetNFTMintAvailable = !!authorizedMinters?.find((address) => address === decodedAddress);

  const sendMessage = useSendMessage(TESTNET_NFT_CONTRACT_ADDRESS, metadata);

  const mintTestnetNFT = () => sendMessage({ Mint: null });

  return { mintTestnetNFT, isTestnetNFTMintAvailable };
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

export { useNFTsState, useNFTs, useNFTSearch, useTestnetNFT, useTestnetAutoLogin };
