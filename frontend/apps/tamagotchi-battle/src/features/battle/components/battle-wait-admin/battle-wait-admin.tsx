import { buttonStyles } from '@gear-js/ui';
import { SpriteIcon } from 'components/ui/sprite-icon';
import { useBattle } from '../../context';
import { useBattleMessage } from '../../hooks';
import { cn } from 'app/utils';
import { useAccount } from '@gear-js/react-hooks';
import { useFetchVoucher } from 'app/hooks/use-fetch-voucher';
import { useCheckBalance } from 'features/wallet/hooks';
import { GAS_LIMIT } from 'app/consts';

export const BattleWaitAdmin = () => {
  const { players, isPending, setIsPending } = useBattle();
  const handleMessage = useBattleMessage();
  const { account } = useAccount();
  const { isVoucher, isLoading } = useFetchVoucher(account?.address);
  const { checkBalance } = useCheckBalance(isVoucher);

  const handler = async () => {
    const payload = { StartBattle: null };
    const onSuccess = () => setIsPending(false);
    const onError = () => setIsPending(false);

    setIsPending(true);

    checkBalance(
      GAS_LIMIT,
      () => {
        handleMessage({ payload, onSuccess, onError, withVoucher: isVoucher });
      },
      onError,
    );
  };

  return (
    <section className="text-center m-auto">
      <div className="max-w-[368px] mt-6 m-auto">
        <p className="font-kanit text-base text-white/80 tracking-wider">
          Participants connected:{' '}
          <b className="inline-block ml-1 text-xl font-semibold text-white">{players.length} / 50</b>
        </p>
        <div className="mt-12">
          <button
            className={cn(
              'relative btn items-center gap-2 min-w-[250px] transition-colors',
              'before:absolute before:-inset-1 before:border before:border-primary/50 before:rounded-[90px] before:animate-wave-2',
              'after:absolute after:-inset-2 after:border after:border-primary/30 after:rounded-[90px] after:animate-wave',
              buttonStyles.primary,
              buttonStyles.button,
            )}
            onClick={handler}
            disabled={isPending || players.length < 2 || isLoading}>
            <SpriteIcon name="swords" className="w-5 h-5" /> <span>Start Battle</span>
          </button>
        </div>
      </div>
    </section>
  );
};
