import { useEffect, useState, MutableRefObject, RefObject, useCallback, useMemo } from 'react';
import { useSearchParams } from 'react-router-dom';
import { ProgramMetadata, getProgramMetadata } from '@gear-js/api';
import { useAlert, useReadFullState, useSendMessage } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { useAtom } from 'jotai';
import factoryMetaTxt from '@/assets/meta/collection_factory.meta.txt';
import collectionMetaTxt from '@/assets/meta/nft_collection.meta.txt';
import { ADDRESS, LOCAL_STORAGE, SEARCH_PARAMS } from '@/consts';
import { Handler, ProgramStateRes } from '@/types';
import { CONTRACT_ADDRESS_ATOM } from '@/atoms';

function useProgramMetadata(source: string) {
  const alert = useAlert();

  const [metadata, setMetadata] = useState<ProgramMetadata>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.text())
      .then((raw) => `0x${raw}` as HexString)
      .then((metaHex) => getProgramMetadata(metaHex))
      .then((result) => setMetadata(result))
      .catch(({ message }: Error) => alert.error(message));
  }, [source, alert]);

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
      .then((raw) => getProgramMetadata(`0x${raw}`))
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

function useFactoryState() {
  const programId = ADDRESS.FACTORY;
  const meta = useProgramMetadata(factoryMetaTxt);
  const state: ProgramStateRes = useReadFullState(programId, meta);

  return state;
}

function useReadState<T>({ programId, meta }: { programId?: HexString; meta: string }) {
  const metadata = useProgramMetadata(meta);
  return useReadFullState<T>(programId, metadata);
}

function useCreateStreamMetadata() {
  const meta = useMetadata(collectionMetaTxt);

  const memoizedMeta = useMemo(() => meta, [meta]);

  return memoizedMeta;
}

function useCollectionMessage(address: string) {
  const meta = useCreateStreamMetadata();

  const message = useSendMessage(address as HexString, meta);

  return { meta, message };
}

export {
  useProgramMetadata,
  useContractAddressSetup,
  useClickOutside,
  useMetadata,
  useMediaQuery,
  useFactoryState,
  useReadState,
  useCollectionMessage,
};
