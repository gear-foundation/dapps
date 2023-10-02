import { useEffect, useState } from 'react';
import { useAlert } from '@gear-js/react-hooks';
import { getStateMetadata, ProgramMetadata, StateMetadata } from '@gear-js/api';
import { HexString } from '@polkadot/util/types';

function useBuffer(source: string) {
  const alert = useAlert();

  const [buffer, setBuffer] = useState<Buffer>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.arrayBuffer())
      .then((arrayBuffer) => Buffer.from(arrayBuffer))
      .then((result) => setBuffer(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return buffer;
}

function useProgramMetadata(source: string) {
  const alert = useAlert();

  const [metadata, setMetadata] = useState<ProgramMetadata>();

  useEffect(() => {
    fetch(source)
      .then((response) => response.text())
      .then((raw) => `0x${raw}` as HexString)
      .then((metaHex) => ProgramMetadata.from(metaHex))
      .then((result) => setMetadata(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return metadata;
}

function useStateMetadata(wasm: Buffer | undefined) {
  const alert = useAlert();

  const [stateMetadata, setStateMetadata] = useState<StateMetadata>();

  useEffect(() => {
    if (!wasm) return;

    getStateMetadata(wasm)
      .then((result) => setStateMetadata(result))
      .catch(({ message }: Error) => alert.error(message));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [wasm]);

  return stateMetadata;
}

export { useBuffer, useProgramMetadata, useStateMetadata };
