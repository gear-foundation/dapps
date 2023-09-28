import { ProgramMetadata } from '@gear-js/api';
import { HexString } from '@polkadot/util/types';
import { useState, useEffect } from 'react';

function useMetadata(source: string) {
  const [metadata, setMetadata] = useState<ProgramMetadata>();

  useEffect(() => {
    fetch(source)
      .then((result) => result.text())
      .then((text) => `0x${text}` as HexString)
      .then((metaHex) => ProgramMetadata.from(metaHex))
      .then((meta) => setMetadata(meta));
  }, [source]);

  return metadata;
}

export { useMetadata };
