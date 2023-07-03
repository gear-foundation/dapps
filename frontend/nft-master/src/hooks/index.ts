import { ProgramMetadata, getProgramMetadata } from '@gear-js/api';
import { useAlert } from '@gear-js/react-hooks';
import { HexString } from '@polkadot/util/types';
import { useState, useEffect, useRef } from 'react';

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
