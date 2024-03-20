import { HexString, ProgramMetadata, decodeAddress } from '@gear-js/api';
import { useAccount, useBalance, useReadFullState, useVouchers } from '@gear-js/react-hooks';
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

function useVoucherBalance(programId: HexString, address: string | undefined) {
  const decodedAddress = address ? decodeAddress(address) : '';

  const { vouchers } = useVouchers(decodedAddress, programId);

  const [voucherId] = Object.keys(vouchers || {});
  const { balance } = useBalance(voucherId);

  return balance ? balance.toNumber() : 0;
}

function useVoucherId(programId: HexString, address: string | undefined) {
  const decodedAddress = address ? decodeAddress(address) : '';

  const { vouchers } = useVouchers(decodedAddress, programId);
  const [voucherId] = Object.keys(vouchers || {});

  return voucherId as HexString | undefined;
}

export { useSession, useVoucherBalance, useVoucherId };
