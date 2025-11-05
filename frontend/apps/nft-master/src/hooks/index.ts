import { ProgramMetadata } from '@gear-js/api';
import { useAccount, useAlert, withoutCommas } from '@gear-js/react-hooks';
import { stringShorten } from '@polkadot/util';
import { useAtom } from 'jotai';
import { useState, useEffect } from 'react';
import { useQuery } from 'urql';

import { getErrorMessage } from '@dapps-frontend/ui';

import { IS_BALANCE_LOW_ATOM, isPendingUI } from '@/consts';
import { useAccountAvailableBalance } from '@/features/available-balance/hooks';
import { GetAccountNFTQuery } from '@/features/nfts/queries';
import { AccountNftsQueryResult, AccountNftsQueryVariables, NFT } from '@/features/nfts/types';

export function usePendingUI() {
  const [isPending, setIsPending] = useAtom(isPendingUI);
  return { isPending, setIsPending };
}

export function useReadStateFromApi<T = NFT>() {
  const [nft, setNft] = useState<T[] | null>(null);
  const [isStateRead, setIsStateRead] = useState(false);
  const { account } = useAccount();
  const { setIsPending } = usePendingUI();
  const alert = useAlert();
  const [result, reexecuteQuery] = useQuery<AccountNftsQueryResult<T>, AccountNftsQueryVariables>({
    query: GetAccountNFTQuery,
    variables: { account_id: account?.decodedAddress || '' },
  });

  const { data, fetching, error } = result;

  useEffect(() => {
    setIsPending(true);

    if (error) {
      alert.error(error.message);
      setIsPending(false);
      return;
    }

    if (data) {
      const { nfts } = data;

      setNft(nfts ?? null);
    }
    if (!fetching) {
      setIsStateRead(true);
    }
  }, [data, fetching, error, alert, setIsPending]);

  return { state: nft, isStateRead, setIsStateRead, reexecuteQuery };
}

// Set value in seconds
export const sleep = (s: number) => new Promise((resolve) => setTimeout(resolve, s * 1000));

export function useProgramMetadata(source: string) {
  const alert = useAlert();

  const [metadata, setMetadata] = useState<ProgramMetadata>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.text())
      .then((raw) => ProgramMetadata.from(`0x${raw}`))
      .then((result) => setMetadata(result))
      .catch((error) => alert.error(getErrorMessage(error)));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return metadata;
}

export const useResizeEffect = (callback: () => void) => {
  useEffect(() => {
    window.addEventListener('resize', callback);

    return () => {
      window.removeEventListener('resize', callback);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
};

export function useCheckBalance() {
  const { account } = useAccount();
  const { availableBalance } = useAccountAvailableBalance();
  const alert = useAlert();
  const [isLowBalance, setIsLowBalance] = useAtom(IS_BALANCE_LOW_ATOM);

  const checkBalance = (payload: () => void, onError?: () => void) => {
    if (isLowBalance) {
      alert.error(`Low balance on ${stringShorten(account?.decodedAddress || '', 8)}`);

      if (onError) {
        onError();
      }

      return;
    }

    payload();
  };

  useEffect(() => {
    if (
      availableBalance &&
      Number(withoutCommas(availableBalance.value)) < Number(withoutCommas(availableBalance.existentialDeposit)) + 5
    ) {
      setIsLowBalance(true);
    } else {
      setIsLowBalance(false);
    }
  }, [availableBalance, setIsLowBalance]);

  const getIsLowBalance = () =>
    availableBalance &&
    Number(withoutCommas(availableBalance.value)) < Number(withoutCommas(availableBalance.existentialDeposit)) + 5;

  return { checkBalance, getIsLowBalance, isLowBalance };
}
