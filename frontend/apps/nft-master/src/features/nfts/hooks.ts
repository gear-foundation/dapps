import { decodeAddress } from '@gear-js/api';
import { useAccount, useAlert, useHandleCalculateGas, useSendMessage, withoutCommas } from '@gear-js/react-hooks';
import { useCallback, useEffect, useMemo } from 'react';
import { useAtom } from 'jotai';
import { useMutation } from 'urql';
import { socket } from 'utils';
import metaMasterNFT from 'assets/master_nft.meta.txt';
import { sleep, usePendingUI, useProgramMetadata, useReadStateFromApi } from 'hooks';
import { useSearchParams } from 'react-router-dom';
import { IUserNFTRequest, NFT } from './types';
import { IS_MINTING_ATOM, NFTS_ATOM, USER_NFT_QUERY_ATOM } from './consts';
import { ADDRESS } from '../../consts';

const programId = ADDRESS.MASTER_CONTRACT;

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

  useEffect(() => {
    if (!searchQuery) {
      resetSearchQuery();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchQuery]);

  return { searchQuery, decodedQueryAddress, resetSearchQuery };
}

export function useNFTs() {
  const [NFTs, setNFTs] = useAtom(NFTS_ATOM);
  const [userNftQuery, setUserNftQuery] = useAtom(USER_NFT_QUERY_ATOM);

  const getIpfsAddress = (cid: string) => `${ADDRESS.IPFS_GATEWAY}/${cid}`;

  const getImageUrl = (value: string) => (value.startsWith('https://') ? value : getIpfsAddress(value));

  return {
    nfts: NFTs || [],
    setNFTs,
    getImageUrl,
    getIpfsAddress,
    userNftQuery,
    setUserNftQuery,
  };
}

export function useMintNFT() {
  const { nfts, userNftQuery } = useNFTs();
  const { account } = useAccount();
  const alert = useAlert();
  const masterMetadata = useProgramMetadata(metaMasterNFT);
  const sendMessage = useSendMessage(programId, masterMetadata);
  const calculateGas = useHandleCalculateGas(programId, masterMetadata);
  const [isMinting, setIsMinting] = useAtom(IS_MINTING_ATOM);
  const { setIsPending } = usePendingUI();

  const hasNFT = Boolean(nfts.find(({ owner }) => owner.id === account?.decodedAddress));

  const mintNFT = () => {
    if (isMinting || !masterMetadata || !programId || !account) {
      return;
    }

    setIsMinting(true);

    const payload = {
      Mint: null,
    };

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const limit = withoutCommas(min_limit as string);
        sendMessage({
          payload,
          gasLimit: Math.floor(Number(limit) + Number(limit) * 0.2),
          onSuccess: () => {
            setIsPending(true);
            sleep(5).then(() => {
              if (userNftQuery.fn) {
                userNftQuery.fn({ requestPolicy: 'network-only' });
              }
            });
            setIsMinting(false);
          },
          onError: () => {
            setIsMinting(false);
            alert.error('Error when minting NFT');
          },
        });
      })
      .catch((error) => {
        console.log(error);
        alert.error('Gas calculation error');
        setIsMinting(false);
      });
  };

  return {
    mintNFT,
    isMinting,
    isMintingAvailable: !hasNFT,
  };
}

export function useNFTSetup() {
  const { setNFTs, setUserNftQuery } = useNFTs();
  const { setIsPending } = usePendingUI();
  const { state, isStateRead, setIsStateRead, reexecuteQuery } = useReadStateFromApi<IUserNFTRequest | null>();

  const reex = useCallback(reexecuteQuery, [reexecuteQuery]);

  useEffect(() => {
    if (reex) {
      setUserNftQuery({ fn: reex });
    }
  }, [setUserNftQuery, reex]);

  useEffect(() => {
    if (isStateRead) {
      setNFTs(state ? (state as NFT[]) : []);
      setIsPending(false);
      setIsStateRead(false);
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [state, isStateRead]);

  return typeof state !== 'undefined';
}
