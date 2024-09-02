import { useEffect, useState, MutableRefObject, RefObject, useCallback, useMemo } from 'react';
import { useLocation, useSearchParams } from 'react-router-dom';
import { ProgramMetadata, StateMetadata, getStateMetadata } from '@gear-js/api';
import {
  useAccount,
  useAlert,
  useReadFullState,
  withoutCommas,
  useHandleCalculateGas as useCalculateGasNative,
} from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { AnyJson, AnyNumber } from '@polkadot/types/types';
import { useDnsProgramIds } from '@dapps-frontend/hooks';
import metaTxt from '@/assets/meta/meta.txt';
import { ACCOUNT_ID_LOCAL_STORAGE_KEY, LOCAL_STORAGE, SEARCH_PARAMS } from '@/consts';
import { Handler, ProgramStateRes } from '@/types';
import { AUTH_TOKEN_LOCAL_STORAGE_KEY } from './features/Auth/consts';
import { useAccountAvailableBalance } from './features/Wallet/hooks';

export function useProgramMetadata(source: string) {
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

export function useContractAddressSetup() {
  const [searchParams, setSearchParams] = useSearchParams();
  const { programId } = useDnsProgramIds();

  useEffect(() => {
    if (!programId) return;

    localStorage.setItem(LOCAL_STORAGE.CONTRACT_ADDRESS, programId);

    searchParams.set(SEARCH_PARAMS.MASTER_CONTRACT_ID, programId);
    setSearchParams(searchParams);
  }, [programId, searchParams, setSearchParams]);
}

// @deprecated
export function useClickOutside(
  handler: Handler,
  ...refs: (RefObject<HTMLElement> | MutableRefObject<HTMLElement>)[]
): void {
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

export function useStateMetadata(source: string) {
  const alert = useAlert();

  const [stateMetadata, setStateMetadata] = useState<StateMetadata>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) => Buffer.from(arrayBuffer))
      .then((buffer) => getStateMetadata(buffer))
      .then((result) => setStateMetadata(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return stateMetadata;
}

export function useMediaQuery(width: number) {
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

export function useProgramState<T>(payload?: AnyJson) {
  const { programId } = useDnsProgramIds();
  const meta = useProgramMetadata(metaTxt);
  const state: ProgramStateRes<T> = useReadFullState(programId, meta, payload);

  return state;
}

export function useReadState<T>({
  programId,
  meta,
  payload,
}: {
  programId?: HexString;
  meta: string;
  payload?: AnyJson;
}) {
  const metadata = useProgramMetadata(meta);
  return useReadFullState<T>(programId, metadata, payload);
}

export function useLoginByParams() {
  const { search } = useLocation();
  const { login, isAccountReady, wallets } = useAccount();

  const query = useMemo(() => new URLSearchParams(search), [search]);

  useEffect(() => {
    // TODO: auth is not needed, remove!
    const isAccount = localStorage.getItem(ACCOUNT_ID_LOCAL_STORAGE_KEY);
    const isAuthToken = localStorage.getItem(AUTH_TOKEN_LOCAL_STORAGE_KEY);

    if (isAccountReady && query.size) {
      const account = query.get(ACCOUNT_ID_LOCAL_STORAGE_KEY);
      const authToken = query.get(AUTH_TOKEN_LOCAL_STORAGE_KEY);

      const isEmptyStorage = !isAuthToken && !isAccount;
      const isSameUser = isAccount === account;
      const isDataCorrect = account && authToken;

      if ((isEmptyStorage || !isSameUser) && !!isDataCorrect) {
        localStorage.setItem(ACCOUNT_ID_LOCAL_STORAGE_KEY, account);
        localStorage.setItem(AUTH_TOKEN_LOCAL_STORAGE_KEY, authToken);

        const candidate = Object.values(wallets || {})
          .flatMap(({ accounts }) => accounts)
          .find((acc) => acc?.address === account);

        if (candidate) login(candidate);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isAccountReady, wallets, query]);
}

export const useHandleCalculateGas = (address: HexString, meta?: ProgramMetadata) => {
  const { availableBalance } = useAccountAvailableBalance();
  const calculateGasNative = useCalculateGasNative(address, meta);

  const alert = useAlert();

  return (initPayload: AnyJson, value?: AnyNumber | undefined) => {
    const balance = Number(withoutCommas(availableBalance?.value || ''));
    const existentialDeposit = Number(withoutCommas(availableBalance?.existentialDeposit || ''));

    if (!balance || balance < existentialDeposit) {
      alert.error(`Low balance when calculating gas`);
    }

    return calculateGasNative(initPayload, value);
  };
};
