import clsx from 'clsx';
import { buttonStyles } from '@gear-js/ui';
import { Icon } from '../../ui/icon';
import { useApp, useBattle } from 'app/context';
import { useBattleMessage } from '../../../app/hooks/use-battle';

export const BattleWaitAdmin = () => {
  const { isPending, setIsPending } = useApp();
  const { battleState: battle } = useBattle();
  const handleMessage = useBattleMessage();

  const handler = () => {
    setIsPending(true);
    handleMessage(
      { StartBattle: null },
      {
        onSuccess: () => {
          setIsPending(false);
          console.log('Battle started');
        },
        onError: () => {
          setIsPending(false);
          console.log('Failed to initialize');
        },
      },
    );
  };

  return (
    <section className="text-center m-auto">
      <div className="max-w-[368px] mt-6 m-auto">
        <p className="text-base text-white/80">
          Participants connected:{' '}
          <b className="inline-block ml-1 text-xl font-semibold tracking-wider text-white">
            {battle ? Object.keys(battle.players).length : 0} / 48
          </b>
        </p>
        <div className="mt-12">
          <button
            className={clsx('btn items-center gap-2 min-w-[250px] transition-colors', buttonStyles.primary)}
            onClick={handler}
            disabled={isPending}>
            <Icon name="swords" className="w-5 h-5" /> <span>Start Battle</span>
          </button>
        </div>
      </div>
    </section>
  );
};
