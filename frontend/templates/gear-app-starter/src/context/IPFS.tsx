import { create, IPFSHTTPClient } from 'ipfs-http-client';
import { createContext, ReactNode, useRef } from 'react';

type Props = {
  children: ReactNode;
};

const IPFSContext = createContext({} as IPFSHTTPClient);

function IPFSProvider({ children }: Props) {    
  const ipfsRef = useRef(create({ url: process.env.REACT_APP_IPFS_ADDRESS as string }));
  const { Provider } = IPFSContext;

  return <Provider value={ipfsRef.current}>{children}</Provider>;
}

export { IPFSContext, IPFSProvider };