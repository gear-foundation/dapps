import { MessagesDispatched, decodeAddress, getProgramMetadata, getStateMetadata } from '@gear-js/api';
import { useAccount, useAlert, useApi, useSendMessage } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { UnsubscribePromise } from '@polkadot/api/types';
import { useEffect, useMemo, useState } from 'react';
import { atom, useAtom } from 'jotai';
import metaMasterNFT from 'assets/master_nft.meta.txt';
import metaTxt from 'assets/nft_master.meta.txt';
import metaWasm from 'assets/market_nft_state.meta.wasm';
import { useProgramMetadata, useStateMetadata } from 'hooks';
import { useLocation, useSearchParams } from 'react-router-dom';
import { useNodeAddress } from 'features/node-switch';
import { isHex } from '@polkadot/util';
import { useContractAddress } from '../contract-address';
import { MasterContractState, NFTContractState } from './types';
import { NFTS_ATOM, NFT_CONTRACTS_ATOM, TESTNET_NFT_CONTRACT_ADDRESS } from './consts';
// import { useSendMessage } from '../../hooks/useSendMessage';

const handleStateChange = ({ data }: MessagesDispatched, programId: HexString, onChange: () => void) => {
  const changedIDs = data.stateChanges.toHuman() as HexString[];
  const isAnyChange = changedIDs.some((id) => id === programId);

  if (isAnyChange) onChange();
};

export function useNFTSearch() {
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

export function useGetAllNFTs() {
  const { api } = useApi();
  const alert = useAlert();

  const [NFTContracts] = useAtom(NFT_CONTRACTS_ATOM);
  const [, setNFTs] = useAtom(NFTS_ATOM);
  const [isStateRead, setIsStateRead] = useState<boolean>(false);

  const getAllNFTs = (cb?: () => void) => {
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
      .then((result) => {
        setIsStateRead(true);
        setNFTs(result.flat());
      })
      .catch(({ message }: Error) => alert.error(message))
      .finally(() => cb && cb());
  };

  return { getAllNFTs, isStateRead };
}

export function useNFTsState() {
  const { api, isApiReady } = useApi();
  const { account, isAccountReady } = useAccount();
  const alert = useAlert();
  const metawasm = useStateMetadata(metaWasm);
  const { pathname } = useLocation();
  const { isTestnet } = useNodeAddress();
  const { contractAddress: masterContractAddress } = useContractAddress();
  const masterMetadata = useProgramMetadata(metaTxt);

  const { searchQuery } = useNFTSearch();
  const { getAllNFTs } = useGetAllNFTs();

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
    if (!isApiReady || !isAccountReady || !masterContractAddress || !masterMetadata) return;

    readMasterContractState();

    const unsub = api.gearEvents.subscribeToGearEvent('MessagesDispatched', (event) =>
      handleStateChange(event, masterContractAddress, readMasterContractState),
    );

    return () => {
      unsub.then((unsubCallback) => unsubCallback());
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, isAccountReady, masterContractAddress, masterMetadata]);

  const getUserNFT = (storage: [HexString, string]) => {
    const [programId, metaRaw] = storage;
    const metadata = getProgramMetadata(`0x${metaRaw}`);

    api.programState
      .read({ programId }, metadata)
      .then((codec) => codec.toHuman() as NFTContractState)
      .then(({ tokens, collection }) =>
        tokens.map(([id, token]) => ({ ...token, id, programId, collection: collection.name })),
      )
      .then((result) => setNFTs(result.flat()))
      .catch(({ message }: Error) => alert.error(message));
  };

  const getTestnetNFTs = () => {
    if (!NFTContracts || !isTestnet || !metawasm?.buffer) return;

    getStateMetadata(metawasm?.buffer)
      .then((stateMetadata) =>
        api.programState.readUsingWasm(
          {
            programId: TESTNET_NFT_CONTRACT_ADDRESS,
            wasm: metawasm?.buffer,
            fn_name: 'get_storage_id',
            argument: account?.decodedAddress,
          },
          stateMetadata,
        ),
      )
      .then((storageId) => {
        const userStorage = NFTContracts.find(([address]) => storageId.toHuman() === address);

        if (userStorage && !searchQuery) {
          const nftStorageId = pathname.slice(1, pathname.indexOf('/', 1));
          const isViewingNFT = isHex(nftStorageId);
          const [programId] = userStorage;
          if (isViewingNFT) {
            if (nftStorageId === programId) getUserNFT(userStorage);
            else getAllNFTs();
          } else getUserNFT(userStorage);
        } else getAllNFTs();
      })
      .catch(({ message }: Error) => alert.error(message));
  };

  const readNFTContractsStates = () => {
    if (!NFTContracts) return;

    if (isTestnet && metawasm?.buffer) {
      // is user is logged in, read only his nft storage state
      if (account) getTestnetNFTs();
      else getAllNFTs();
    } else getAllNFTs();
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
  }, [NFTContracts, account?.decodedAddress]);

  return masterContractAddress ? !!NFTs : true;
}

export function useNFTs() {
  const [NFTs] = useAtom(NFTS_ATOM);
  const [NFTContracts] = useAtom(NFT_CONTRACTS_ATOM);

  return { nfts: NFTs || [], NFTContracts: NFTContracts || [] };
}

const TESTNET_NFT_IS_MINTER_ATOM = atom<boolean | undefined>(undefined);

export function useTestnetNFTSetup() {
  const { api, isApiReady } = useApi();
  const { account } = useAccount();
  const { isTestnet } = useNodeAddress();
  const alert = useAlert();
  const metawasm = useStateMetadata(metaWasm);
  const [isMinter, setIsMinter] = useAtom(TESTNET_NFT_IS_MINTER_ATOM);

  const readState = () => {
    if (!api || !metawasm) return;

    getStateMetadata(metawasm?.buffer)
      .then((stateMetadata) =>
        api.programState.readUsingWasm(
          {
            programId: TESTNET_NFT_CONTRACT_ADDRESS,
            wasm: metawasm?.buffer,
            fn_name: 'in_minter_list',
            argument: account?.decodedAddress,
          },
          stateMetadata,
        ),
      )
      .then((result) => setIsMinter(result.toHuman() as boolean))
      .catch(({ message }: Error) => alert.error(message));
  };

  useEffect(() => {
    if (!isTestnet || !isApiReady || !metawasm) return;

    readState();

    const unsub = api.gearEvents.subscribeToGearEvent('MessagesDispatched', (event) =>
      handleStateChange(event, TESTNET_NFT_CONTRACT_ADDRESS, readState),
    );

    return () => {
      unsub.then((unsubCallback) => unsubCallback());
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isApiReady, isTestnet, account?.decodedAddress]);

  return isTestnet ? typeof isMinter !== 'undefined' : true;
}

export function useTestnetNFT() {
  const { nfts } = useNFTs();
  const { isTestnet } = useNodeAddress();
  const { account } = useAccount();
  const metadata = useProgramMetadata(metaMasterNFT);
  const sendMessage = useSendMessage(TESTNET_NFT_CONTRACT_ADDRESS, metadata, { isMaxGasLimit: true });
  const [isMinter] = useAtom(TESTNET_NFT_IS_MINTER_ATOM);

  const [isMinting, setIsMinting] = useState(false);
  const hasNFT = Boolean(nfts.find(({ owner }) => owner === account?.decodedAddress));
  const { getAllNFTs, isStateRead } = useGetAllNFTs();

  const mintTestnetNFT = () => {
    setIsMinting(true);

    if (!isStateRead) getAllNFTs();

    sendMessage({ Mint: null }, { onSuccess: () => setIsMinting(false), onError: () => setIsMinting(false) });
  };

  return {
    isMinting,
    mintTestnetNFT,
    isTestnetNFTMintAvailable: isTestnet ? !!(isMinter && !hasNFT) : false,
  };
}

export function useTestnetAutoLogin() {
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
