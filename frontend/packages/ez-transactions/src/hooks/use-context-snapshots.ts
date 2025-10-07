import { useRef, useEffect } from 'react';

import type { GaslessContext } from '../features/gasless-transactions/context/types';
import type { SignlessContext } from '../features/signless-transactions/context/types';

type SignlessSnapshot = Pick<
  SignlessContext,
  'pair' | 'voucher' | 'isActive' | 'isSessionActive' | 'isSessionReady' | 'openSessionModal'
>;

type GaslessSnapshot = Pick<GaslessContext, 'voucherId' | 'isEnabled' | 'requestVoucher' | 'isActive'>;

export const useContextSnapshots = (signless: SignlessContext, gasless: GaslessContext) => {
  const signlessSnapshotRef = useRef<SignlessSnapshot>({
    pair: signless.pair,
    voucher: signless.voucher,
    isActive: signless.isActive,
    isSessionActive: signless.isSessionActive,
    isSessionReady: signless.isSessionReady,
    openSessionModal: signless.openSessionModal,
  });

  const gaslessSnapshotRef = useRef<GaslessSnapshot>({
    voucherId: gasless.voucherId,
    isEnabled: gasless.isEnabled,
    requestVoucher: gasless.requestVoucher,
    isActive: gasless.isActive,
  });

  useEffect(() => {
    signlessSnapshotRef.current = {
      pair: signless.pair,
      voucher: signless.voucher,
      isActive: signless.isActive,
      isSessionActive: signless.isSessionActive,
      isSessionReady: signless.isSessionReady,
      openSessionModal: signless.openSessionModal,
    };
  }, [
    signless.pair,
    signless.voucher,
    signless.isActive,
    signless.isSessionActive,
    signless.isSessionReady,
    signless.openSessionModal,
  ]);

  useEffect(() => {
    gaslessSnapshotRef.current = {
      voucherId: gasless.voucherId,
      isEnabled: gasless.isEnabled,
      requestVoucher: gasless.requestVoucher,
      isActive: gasless.isActive,
    };
  }, [gasless.voucherId, gasless.isEnabled, gasless.requestVoucher, gasless.isActive]);

  return { signlessSnapshotRef, gaslessSnapshotRef };
};

export type { SignlessSnapshot, GaslessSnapshot };
