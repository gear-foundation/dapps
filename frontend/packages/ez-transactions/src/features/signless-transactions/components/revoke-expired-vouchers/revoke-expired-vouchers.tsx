import { HexString } from '@gear-js/api';
import { useAccount, useAlert, useApi, useBalanceFormat } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useMemo } from 'react';

import PowerSVG from '@ez/assets/icons/power.svg?react';

import { useSignlessTransactions } from '../../context';
import { useBatchSignAndSend } from '../../hooks/use-batch-sign-and-send';

import styles from './revoke-expired-vouchers.module.css';

type ExpiredVoucher = {
  id: HexString;
  balance: bigint;
};

function RevokeExpiredVouchers() {
  const { account } = useAccount();
  const { api, isApiReady } = useApi();
  const alert = useAlert();
  const { getFormattedBalance } = useBalanceFormat();
  const { batchSignAndSend } = useBatchSignAndSend('all');
  const queryClient = useQueryClient();
  const { storagePair } = useSignlessTransactions();

  const { data: expiredVouchers = [], isLoading: isLoadingVouchers } = useQuery({
    queryKey: ['expiredVouchers', storagePair?.address, account?.decodedAddress],
    queryFn: async (): Promise<ExpiredVoucher[]> => {
      if (!isApiReady || !account || !storagePair) return [];

      const { block } = await api.rpc.chain.getBlock();
      const currentBlockNumber = block.header.number.toNumber();

      const allExpiredVouchersWithoutBalance: Omit<ExpiredVoucher, 'balance'>[] = [];

      try {
        const vouchers = await api.voucher.getAllForAccount(storagePair.address);
        Object.entries(vouchers).forEach(([id, voucher]) => {
          const isExpired = currentBlockNumber > voucher.expiry;
          const isOwner = account.decodedAddress === voucher.owner;

          if (isExpired && isOwner) {
            allExpiredVouchersWithoutBalance.push({ id: id as HexString });
          }
        });
      } catch (error) {
        console.error(`Error fetching vouchers:`, error);
      }

      // Fetch all balances in parallel
      const vouchersWithBalances = await Promise.all(
        allExpiredVouchersWithoutBalance.map(async (voucher) => {
          try {
            const balance = (await api.balance.findOut(voucher.id)).toBigInt();
            return { ...voucher, balance };
          } catch (error) {
            console.error(`Error fetching balance for voucher ${voucher.id}:`, error);
            return { ...voucher, balance: 0n };
          }
        }),
      );

      return vouchersWithBalances.filter((voucher) => voucher.balance > 0n);
    },
    enabled: isApiReady && !!account && !!storagePair,
  });

  const { mutate: revokeVouchers, isPending: isRevoking } = useMutation({
    mutationFn: async (vouchers: ExpiredVoucher[]) => {
      if (!api) throw new Error('API is not initialized');
      if (!storagePair) return;
      const revokeExtrinsics = vouchers.map((voucher) => api.voucher.revoke(storagePair.address, voucher.id));

      return new Promise<void>((resolve, reject) => {
        void batchSignAndSend(revokeExtrinsics, {
          onSuccess: () => {
            resolve();
          },
          onError: (error) => {
            reject(new Error(error));
          },
        });
      });
    },
    onSuccess: () => {
      alert.success('Expired vouchers revoked successfully');
      void queryClient.invalidateQueries({ queryKey: ['expiredVouchers'] });
    },
    onError: (error) => {
      const errorMessage = error instanceof Error ? error.message : String(error);
      alert.error(errorMessage);
    },
  });

  const totalBalance = useMemo(() => {
    return expiredVouchers.reduce((sum, voucher) => sum + voucher.balance, 0n);
  }, [expiredVouchers]);

  const formattedBalance = useMemo(() => {
    if (totalBalance === 0n) return '0';
    return getFormattedBalance(totalBalance).value;
  }, [totalBalance, getFormattedBalance]);

  const handleRevokeVouchers = () => {
    if (!isApiReady || expiredVouchers.length === 0) return;
    revokeVouchers(expiredVouchers);
  };

  if (!account) {
    return null;
  }

  const hasBalance = totalBalance > 0n;
  const isDisabled = isLoadingVouchers || isRevoking || !hasBalance;

  const getSubtitle = () => {
    if (isLoadingVouchers) return 'Loading vouchers...';
    if (!hasBalance) return 'No expired vouchers';
    return (
      <span className={styles.balance}>
        {expiredVouchers.length} voucher{expiredVouchers.length > 1 ? 's' : ''} - {formattedBalance} VARA
      </span>
    );
  };

  return (
    <div>
      <Button color="transparent" disabled={isDisabled} className={styles.revokeButton} onClick={handleRevokeVouchers}>
        <PowerSVG />
        <span className={styles.wrapper}>
          Revoke expired vouchers
          <span className={styles.subtitle}>{getSubtitle()}</span>
        </span>
      </Button>
    </div>
  );
}

export { RevokeExpiredVouchers };
