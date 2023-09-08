import { useEffect, useState } from 'react';
import {
  getProgramMetadata,
  ProgramMetadata
} from '@gear-js/api';
import { Buffer } from 'buffer';
import { useAlert } from '@gear-js/react-hooks';

export const useMetadata = (source: RequestInfo | URL) => {
  const [data, setData] = useState<ProgramMetadata>();

  useEffect(() => {
    fetch(source)
      .then((res) => res.text() as Promise<string>)
      .then((raw) => getProgramMetadata(`0x${raw}`))
      .then((meta) => setData(meta));
  }, [source]);

  return data;
};

export const useWasmMetadata = (source: RequestInfo | URL) => {
  const alert = useAlert();
  const [data, setData] = useState<Buffer>();

  useEffect(() => {
    if (source) {
      fetch(source)
        .then((response) => response.arrayBuffer())
        .then((array) => Buffer.from(array))
        .then((buffer) => setData(buffer))
        .catch(({ message }: Error) => alert.error(`Fetch error: ${message}`));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [source]);

  return { buffer: data };
};
