import { HexString } from '@gear-js/api';
import { useAlert, useApi } from '@gear-js/react-hooks';
import { createContext, useEffect, useState, ReactNode, PropsWithChildren } from 'react';

import { SailsProgram } from './lib';

export type DnsContextValue = Record<string, HexString>;

export type DefaultDnsValueName = 'programId';

export type DnsProviderProps<T extends string = DefaultDnsValueName> = {
  names: Record<T, string>;
  dnsContractAddress: `0x${string}`;
  fallback?: ReactNode;
};

const DnsContext = createContext<DnsContextValue>({});

function DnsProvider<T extends string = DefaultDnsValueName>({
  children,
  names,
  dnsContractAddress,
  fallback,
}: PropsWithChildren<DnsProviderProps<T>>) {
  const [programIds, setProgramIds] = useState<DnsContextValue>({});
  const { api, isApiReady } = useApi();
  const alert = useAlert();

  useEffect(() => {
    const init = async () => {
      if (!isApiReady || !api) return;

      if (!dnsContractAddress || !names) {
        throw new Error('dnsContractAddress or names is undefined');
      }

      try {
        const program = new SailsProgram(api, dnsContractAddress);

        const promises = Object.entries<string>(names).map(async ([key, name]) => {
          const info = await program.dns.getContractInfoByName(name).call();

          if (!info) {
            throw new Error(`DNS name not found: ${name}`);
          }

          return { [key]: info.program_id };
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

    void init();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [names, dnsContractAddress, api, isApiReady]);

  return (
    <DnsContext.Provider value={programIds}>{Object.keys(programIds).length ? children : fallback}</DnsContext.Provider>
  );
}

export { DnsContext, DnsProvider };
