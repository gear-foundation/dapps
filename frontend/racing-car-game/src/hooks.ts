import { useEffect, useState, MutableRefObject, RefObject, useCallback, useMemo } from 'react';
import { useLocation, useSearchParams } from 'react-router-dom';
import { ProgramMetadata, StateMetadata, getStateMetadata } from '@gear-js/api';
import {
  useAccount,
  useAlert,
  useReadFullState,
  withoutCommas,
  useHandleCalculateGas as useCalculateGasNative,
  useApi,
  useBalanceFormat,
} from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { AnyJson, AnyNumber } from '@polkadot/types/types';
import { stringShorten } from '@polkadot/util';
import { useAtom, useAtomValue, useSetAtom } from 'jotai';
import metaTxt from '@/assets/meta/meta.txt';
import { ACCOUNT_ID_LOCAL_STORAGE_KEY, ADDRESS, LOCAL_STORAGE, SEARCH_PARAMS } from '@/consts';
import { Handler, INodeSection, ProgramStateRes } from '@/types';
import { CONTRACT_ADDRESS_ATOM, nodesAtom } from '@/atoms';
import { WALLET_ID_LOCAL_STORAGE_KEY } from './features/Wallet/consts';
import { AUTH_TOKEN_LOCAL_STORAGE_KEY } from './features/Auth/consts';
import { get } from './utils';
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

export function useContractAddress() {
  const [address] = useAtom(CONTRACT_ADDRESS_ATOM);

  return address;
}

export function useContractAddressSetup() {
  const [searchParams, setSearchParams] = useSearchParams();

  const address = useContractAddress();

  useEffect(() => {
    if (!address) return;

    localStorage.setItem(LOCAL_STORAGE.CONTRACT_ADDRESS, address);

    searchParams.set(SEARCH_PARAMS.MASTER_CONTRACT_ID, address);
    setSearchParams(searchParams);
  }, [address, searchParams, setSearchParams]);
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
  const programId = ADDRESS.CONTRACT;
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
  const { login, accounts } = useAccount();

  const query = useMemo(() => new URLSearchParams(search), [search]);

  useEffect(() => {
    const isAccount = localStorage.getItem(ACCOUNT_ID_LOCAL_STORAGE_KEY);
    const isWallet = localStorage.getItem(WALLET_ID_LOCAL_STORAGE_KEY);
    const isAuthToken = localStorage.getItem(AUTH_TOKEN_LOCAL_STORAGE_KEY);

    if (query.size) {
      const account = query.get(ACCOUNT_ID_LOCAL_STORAGE_KEY);
      const wallet = query.get(WALLET_ID_LOCAL_STORAGE_KEY);
      const authToken = query.get(AUTH_TOKEN_LOCAL_STORAGE_KEY);

      const isEmptyStorage = !isAuthToken && !isAccount && !isWallet;
      const isSameUser = isAccount === account;
      const isDataCorrect = account && authToken && wallet;

      if ((isEmptyStorage || !isSameUser) && !!isDataCorrect) {
        localStorage.setItem(ACCOUNT_ID_LOCAL_STORAGE_KEY, account);
        localStorage.setItem(WALLET_ID_LOCAL_STORAGE_KEY, wallet);
        localStorage.setItem(AUTH_TOKEN_LOCAL_STORAGE_KEY, authToken);

        const candidate = accounts?.find((a) => a.address === account);
        if (candidate) login(candidate);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [accounts, query]);
}

export function useNodes() {
  const nodes = useAtomValue(nodesAtom);
  const setNodes = useSetAtom(nodesAtom);

  return { nodes, setNodes };
}

export function useNodesSync() {
  const alert = useAlert();
  const { setNodes } = useNodes();
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    setLoading(true);

    const getNodes = async () => {
      try {
        const res1 = await get<INodeSection[]>(ADDRESS.BASE_NODES);
        const res2 = await get<INodeSection[]>(ADDRESS.STAGING_NODES);
        const merged = [...res1, ...res2].map((n) => n.nodes.map((node) => ({ ...node, caption: n.caption }))).flat();

        const nodes = [...new Map(merged.map((o) => [o.address, o])).values()];

        setNodes(nodes);
        // console.log({ nodes })
      } catch (e) {
        alert.error((e as any).message);
      } finally {
        setLoading(false);
      }
    };
    getNodes();

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
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
