import { useContext } from 'react';
import { DnsContext } from '../providers/dns-provider';

function useDnsProgramId() {
  const { programId } = useContext(DnsContext);

  return programId!;
}

export { useDnsProgramId };
