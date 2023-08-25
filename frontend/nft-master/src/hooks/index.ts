import { ProgramMetadata, getProgramMetadata, StateMetadata, getStateMetadata } from '@gear-js/api';
import { useAlert, useReadFullState } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { useState, useEffect, useRef } from 'react';
import { atom, useAtom } from 'jotai';

const isPendingUI = atom<boolean>(false);

export function usePendingUI() {
  const [isPending, setIsPending] = useAtom(isPendingUI);
  return { isPending, setIsPending };
}

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

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return metadata;
}

export function useStateMetadata(source: string) {
  const alert = useAlert();

  const [data, setData] = useState<{
    buffer: Buffer;
    meta: StateMetadata;
  }>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) => Buffer.from(arrayBuffer))
      .then(async (buffer) => ({
        buffer,
        meta: await getStateMetadata(buffer),
      }))
      .then((result) => setData(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return data;
}

export function useReadState<T>({ programId, meta }: { programId?: HexString; meta: string }) {
  const metadata = useProgramMetadata(meta);
  return useReadFullState<T>(programId, metadata);
}

const useOutsideClick = <TElement extends Element>(callback: (event: MouseEvent) => void) => {
  const ref = useRef<TElement>(null);

  const handleClick = (event: MouseEvent) => {
    const isOutsideClick = ref.current && !ref.current.contains(event.target as Node);

    if (isOutsideClick) callback(event);
  };

  useEffect(() => {
    document.addEventListener('click', handleClick);

    return () => document.removeEventListener('click', handleClick);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return ref;
};

const useResizeEffect = (callback: () => void) => {
  useEffect(() => {
    window.addEventListener('resize', callback);

    return () => {
      window.removeEventListener('resize', callback);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
};

export { useProgramMetadata, useOutsideClick, useResizeEffect };
