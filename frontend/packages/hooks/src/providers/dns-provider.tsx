import { HexString } from '@gear-js/api';
import { useAlert } from '@gear-js/react-hooks';
import { createContext, useEffect, useState, ReactNode, PropsWithChildren } from 'react';

export type DnsContextValue = Record<string, HexString>;

export type DefaultDnsValueName = 'programId';

export type DnsProviderProps<T extends string = DefaultDnsValueName> = {
  names: Record<T, string>;
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

const DnsContext = createContext<DnsContextValue>({});

function DnsProvider<T extends string = DefaultDnsValueName>({
  children,
  names,
  dnsApiUrl,
  fallback,
}: PropsWithChildren<DnsProviderProps<T>>) {
  const [programIds, setProgramIds] = useState<DnsContextValue>({});
  const alert = useAlert();

  useEffect(() => {
    const init = async () => {
      if (!dnsApiUrl || !names) {
        throw new Error('dnsApiUrl or names is undefined');
      }
      try {
        const promises = Object.entries(names).map(async ([key, name]) => {
          const response = await fetch(`${dnsApiUrl}/dns/by_name/${name}`);
          const dns: DnsResponse = await response.json();
          return { [key]: dns.address };
        });

        const results = await Promise.all(promises);
        const addresses = results.reduce((acc, current) => ({ ...acc, ...current }), {});

        setProgramIds(addresses);
      } catch (error) {
        const { message } = error as Error;
        alert.error(message);
        console.error(message);
      }
    };

    init();
  }, [names, dnsApiUrl]);

  return (
    <DnsContext.Provider value={programIds}>{Object.keys(programIds).length ? children : fallback}</DnsContext.Provider>
  );
}

export { DnsContext, DnsProvider };
