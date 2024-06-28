import { useContext } from 'react';
import { DnsContext, DefaultDnsValueName } from '../providers/dns-provider';
import { HexString } from '@gear-js/api';

function useDnsProgramIds<T extends string = DefaultDnsValueName>() {
  const context = useContext(DnsContext);

  if (context === undefined) {
    throw new Error('useDnsProgramIds must be used within a DnsProvider');
  }

  return context as Record<T, HexString>;
}

export { useDnsProgramIds };
