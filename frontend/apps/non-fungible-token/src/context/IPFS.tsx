import { create, IPFSHTTPClient } from 'ipfs-http-client';
import { createContext, ReactNode, useRef } from 'react';

import { ENV } from '@/consts';

type Props = {
  children: ReactNode;
};

const IPFSContext = createContext({} as IPFSHTTPClient);

function IPFSProvider({ children }: Props) {
  const ipfsRef = useRef(create({ url: ENV.IPFS }));
  const { Provider } = IPFSContext;

  return <Provider value={ipfsRef.current}>{children}</Provider>;
}

export { IPFSContext, IPFSProvider };
