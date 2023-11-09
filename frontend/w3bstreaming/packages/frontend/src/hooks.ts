import { useEffect, useState, MutableRefObject, RefObject, useCallback } from 'react';
import { useSearchParams } from 'react-router-dom';
import { stringShorten } from '@polkadot/util';
import { AnyJson, AnyNumber } from '@polkadot/types/types';
import { HexString, ProgramMetadata } from '@gear-js/api';
import {
  useAlert,
  useReadFullState,
  useHandleCalculateGas as useCalculateGasNative,
  withoutCommas,
  useApi,
  useAccount,
  useBalanceFormat,
} from '@gear-js/react-hooks';
import { useAtom } from 'jotai';
import metaTxt from '@/assets/meta/meta.txt';
import { ADDRESS, LOCAL_STORAGE, SEARCH_PARAMS } from '@/consts';
import { Handler, ProgramStateRes } from '@/types';
import { CONTRACT_ADDRESS_ATOM } from '@/atoms';
import { useAccountAvailableBalance } from './features/Wallet/hooks';

function useProgramMetadata(source: string) {
  const alert = useAlert();

  const [metadata, setMetadata] = useState<ProgramMetadata>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.text())
      .then((raw) => ProgramMetadata.from(`0x${raw}`))
      .then((result) => setMetadata(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return metadata;
}

function useContractAddress() {
  const [address] = useAtom(CONTRACT_ADDRESS_ATOM);

  return address;
}

function useContractAddressSetup() {
  const [searchParams, setSearchParams] = useSearchParams();

  const address = useContractAddress();

  useEffect(() => {
    if (!address) return;

    localStorage.setItem(LOCAL_STORAGE.CONTRACT_ADDRESS, address);

    searchParams.set(SEARCH_PARAMS.MASTER_CONTRACT_ID, address);
    setSearchParams(searchParams);
  }, [address, searchParams, setSearchParams]);
}

function useClickOutside(handler: Handler, ...refs: (RefObject<HTMLElement> | MutableRefObject<HTMLElement>)[]): void {
  useEffect(() => {
    const listener = (event: Event): void => {
      const existingRefs = refs.filter((item) => item?.current && item);

      const res = existingRefs.every((item) => !item.current?.contains(<Node>event.target));

      if (res) {
        handler(event);
      }
    };

    document.addEventListener('mousedown', listener);

    return (): void => {
      document.removeEventListener('mousedown', listener);
    };
  }, [refs, handler]);
}

function useMetadata(source: RequestInfo | URL) {
  const [data, setData] = useState<ProgramMetadata>();

  useEffect(() => {
    fetch(source)
      .then((res) => res.text() as Promise<string>)
      .then((raw) => ProgramMetadata.from(`0x${raw}`))
      .then((meta) => setData(meta));
  }, [source]);

  return data;
}

function useMediaQuery(width: number) {
  const [targetReached, setTargetReached] = useState(false);

  const updateTarget = useCallback((e: MediaQueryListEvent) => {
    if (e.matches) {
      setTargetReached(true);
    } else {
      setTargetReached(false);
    }
  }, []);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const media = window.matchMedia(`(max-width:${width}px)`);
      media.addEventListener('change', updateTarget);

      if (media.matches) {
        setTargetReached(true);
      }

      return () => media.removeEventListener('change', updateTarget);
    }
  }, [updateTarget, width]);

  return targetReached;
}

function useProgramState() {
  const programId = ADDRESS.CONTRACT;
  const meta = useProgramMetadata(metaTxt);
  const state: ProgramStateRes = useReadFullState(programId, meta, '0x');

  return state;
}

export function useCheckBalance() {
  const { api } = useApi();
  const { account } = useAccount();
  const { availableBalance } = useAccountAvailableBalance();
  const { getChainBalanceValue } = useBalanceFormat();
  const alert = useAlert();

  const checkBalance = (limit: number, callback: () => void, onError?: () => void) => {
    const chainBalance = Number(getChainBalanceValue(Number(withoutCommas(availableBalance?.value || ''))).toFixed());
    const valuePerGas = Number(withoutCommas(api!.valuePerGas!.toHuman()));
    const chainEDeposit = Number(
      getChainBalanceValue(Number(withoutCommas(availableBalance?.existentialDeposit || ''))).toFixed(),
    );

    const chainEDepositWithLimit = chainEDeposit + limit * valuePerGas;
    console.log('LIMIT:');
    console.log(limit);
    console.log(limit * valuePerGas);
    console.log('existentialDeposit:');
    console.log(Number(withoutCommas(availableBalance?.existentialDeposit || '')));
    console.log('eDeposit');
    console.log(chainEDeposit);
    console.log('eDeposit + Limit:');
    console.log(chainEDepositWithLimit);
    console.log('balance:');
    console.log(Number(withoutCommas(availableBalance!.value)));
    console.log('chain balance:');
    console.log(getChainBalanceValue(Number(withoutCommas(availableBalance?.value || ''))).toFixed());
    console.log('low balance: ');
    console.log(chainBalance < chainEDepositWithLimit);

    if (!chainBalance || chainBalance < chainEDepositWithLimit) {
      alert.error(`Low balance on ${stringShorten(account?.decodedAddress || '', 8)}`);

      if (onError) {
        onError();
      }

      return;
    }

    callback();
  };

  return { checkBalance };
}

export const useHandleCalculateGas = (address: HexString, meta?: ProgramMetadata) => {
  const { availableBalance } = useAccountAvailableBalance();
  const calculateGasNative = useCalculateGasNative(address, meta);

  const alert = useAlert();

  return (initPayload: AnyJson, value?: AnyNumber | undefined) => {
    const balance = Number(withoutCommas(availableBalance?.value || ''));
    const existentialDeposit = Number(withoutCommas(availableBalance?.existentialDeposit || ''));
    console.log(balance);
    console.log(existentialDeposit);
    if (!balance || balance < existentialDeposit) {
      alert.error(`Low balance when calculating gas`);
    }

    return calculateGasNative(initPayload, value);
  };
};

export { useProgramMetadata, useContractAddressSetup, useClickOutside, useMetadata, useMediaQuery, useProgramState };
