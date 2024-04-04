import { HexString, ProgramMetadata, decodeAddress } from '@gear-js/api';
import { getTypedEntries, useAccount, useReadFullState, useVouchers } from '@gear-js/react-hooks';
import { useMemo } from 'react';

import { State } from './types';

function useSession(programId: HexString, metadata: ProgramMetadata | undefined) {
  const { account } = useAccount();

  const payload = useMemo(() => ({ SessionForTheAccount: account?.decodedAddress }), [account]);
  const { state } = useReadFullState<State>(programId, metadata, payload);

  const session = state?.SessionForTheAccount;
  const isSessionReady = session !== undefined;

  return { session, isSessionReady };
}

function useLatestVoucher(programId: HexString, address: string | undefined) {
  const decodedAddress = address ? decodeAddress(address) : '';
  const { vouchers } = useVouchers(decodedAddress, programId);

  const typedEntries = getTypedEntries(vouchers || {});

  const latestVoucher = useMemo(() => {
    if (!vouchers || !typedEntries?.length) return undefined;

    const [[id, voucher]] = typedEntries.sort(([, voucher], [, nextVoucher]) => nextVoucher.expiry - voucher.expiry);

    return { ...voucher, id };
  }, [vouchers]);

  return latestVoucher;
}

export { useSession, useLatestVoucher };
