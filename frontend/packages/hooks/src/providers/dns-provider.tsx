import { useAlert } from '@gear-js/react-hooks';
import { PropsWithChildren, createContext, useEffect, useState, ReactNode } from 'react';

type HexString = `0x${string}`;

export type DnsContextProps = {
  programId: HexString;
};

export type DnsProviderProps = {
  name: string;
  dnsApiUrl: string;
  fallback?: ReactNode;
};

export type DnsResponse = {
  id: string;
  name: string;
  address: HexString;
  createdBy: HexString;
  createdAt: string;
  updatedAt: string;
};

export const DnsContext = createContext<Partial<DnsContextProps>>({});

export function DnsProvider({ children, name, dnsApiUrl, fallback }: PropsWithChildren<DnsProviderProps>) {
  const [programId, setProgramId] = useState<HexString>();
  const alert = useAlert();

  useEffect(() => {
    const init = async () => {
      if (!dnsApiUrl || !name) {
        throw new Error('dnsApiUrl or name is undefined');
      }
      try {
        const response = await fetch(`${dnsApiUrl}/dns/by_name/${name}`);
        const dns: DnsResponse = await response.json();

        setProgramId(dns.address);
      } catch (error) {
        const { message } = error as Error;
        alert.error(message);
        console.error(message);
      }
    };

    init();
  }, [name, dnsApiUrl]);

  return <DnsContext.Provider value={{ programId }}>{programId ? children : fallback}</DnsContext.Provider>;
}
