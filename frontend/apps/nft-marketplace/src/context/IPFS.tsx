import { ProviderProps } from '@gear-js/react-hooks';
import { IPFSHTTPClient, create } from 'ipfs-http-client';
import { createContext, useRef } from 'react';

import { ENV } from '@/consts';

const IPFSContext = createContext<IPFSHTTPClient>({} as IPFSHTTPClient);

function IPFSProvider({ children }: ProviderProps) {
  const ipfsRef = useRef(create({ url: ENV.IPFS }));

  const { Provider } = IPFSContext;

  return <Provider value={ipfsRef.current}>{children}</Provider>;
}

export { IPFSContext, IPFSProvider };
