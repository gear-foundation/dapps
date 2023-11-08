import { buttonStyles } from '@gear-js/ui';
import { SpriteIcon } from 'components/ui/sprite-icon';
import { useHandleCalculateGas, withoutCommas } from '@gear-js/react-hooks';
import { BATTLE_ADDRESS } from 'features/battle/consts';
import { useProgramMetadata } from 'app/hooks/api';
import { useBattle } from '../../context';
import { useBattleMessage } from '../../hooks';
import { cn } from 'app/utils';
import metaTxt from '../../assets/meta/battle.meta.txt';

export const BattleWaitAdmin = () => {
  const { players, isPending, setIsPending } = useBattle();
  const handleMessage = useBattleMessage();
  const meta = useProgramMetadata(metaTxt);
  const calculateGas = useHandleCalculateGas(BATTLE_ADDRESS, meta);

  const handler = () => {
    const payload = { StartBattle: null };

    setIsPending(true);

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const limit = withoutCommas(min_limit as string);

        handleMessage({
          payload,
          gasLimit: Math.floor(Number(limit) + Number(limit) * 0.2),
          onSuccess: () => setIsPending(false),
          onError: () => setIsPending(false),
        });
      })
      .catch(() => {
        alert('Gas calculation error');
      });
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
            disabled={isPending || players.length < 2}>
            <SpriteIcon name="swords" className="w-5 h-5" /> <span>Start Battle</span>
          </button>
        </div>
      </div>
    </section>
  );
};
