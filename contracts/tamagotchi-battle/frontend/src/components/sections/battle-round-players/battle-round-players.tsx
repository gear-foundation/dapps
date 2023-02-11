import { useApp, useBattle } from 'app/context';
import clsx from 'clsx';
import { buttonStyles } from '@gear-js/ui';
import { Icon } from 'components/ui/icon';
import { useBattleMessage } from 'app/hooks/use-battle';
import { useEffect, useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import { TamagotchiAvatar } from 'components/common/tamagotchi-avatar';

export const BattleRoundPlayers = () => {
  const { isPending, setIsPending, isAdmin } = useApp();
  const { account } = useAccount();
  const { players, currentPlayer, roundDamage, battleState: battle } = useBattle();
  const [isAllowed, setIsAllowed] = useState<boolean>(false);
  const handleMessage = useBattleMessage();

  useEffect(() => {
    if (battle && account && currentPlayer) {
      setIsAllowed(battle.players[currentPlayer].owner === account.decodedAddress);
    }
  }, [account, battle, currentPlayer, isAdmin]);

  const onSuccess = () => setIsPending(false);
  const onError = () => setIsPending(false);
  const onNewRound = () => {
    setIsPending(true);
    handleMessage({ StartNewRound: null }, { onSuccess, onError });
  };
  const onAttack = () => {
    setIsPending(true);
    handleMessage({ MakeMove: { Attack: null } }, { onSuccess, onError });
  };
  const onDefence = () => {
    setIsPending(true);
    handleMessage({ MakeMove: { Defence: null } }, { onSuccess, onError });
  };

  return (
    <div className="relative grow flex justify-between gap-10 mt-10 xxl:mt-15">
      <div className="relative basis-[40%] w-full flex flex-col">
        <TamagotchiAvatar
          color={players[0].color}
          age={players[0].dateOfBirth}
          className="grow w-full h-full "
          isActive={battle?.state !== 'WaitNextRound' && players[0].tmgId === currentPlayer}
          isWinner={battle?.state === 'WaitNextRound' && battle.currentWinner === players[0].tmgId}
          isDead={!players[0].health}
          damage={roundDamage.length > 0 ? Math.round(roundDamage[0] / 25) : 0}
        />
      </div>
      <div className="absolute top-1/2 left-1/2 z-1 -translate-x-1/2 -translate-y-1/2 flex flex-col gap-6 w-full max-w-[250px]">
        <div className="flex flex-col items-center">
          <p className="text-2xl leading-normal xxl:typo-h2 truncate max-w-[13ch]">
            {currentPlayer && battle?.players[currentPlayer].name}
          </p>
        </div>
        <div className="space-y-3">
          {battle?.state === 'WaitNextRound' && (
            <button
              className={clsx(
                'btn items-center gap-2 w-full transition-colors',
                buttonStyles.primary,
                buttonStyles.button,
              )}
              onClick={onNewRound}
              disabled={isPending || isAdmin ? false : !isAllowed}>
              Start New Round
            </button>
          )}
          {battle?.state === 'GameIsOn' && (
            <>
              <button
                className={clsx(
                  'btn items-center gap-2 w-full bg-error text-white hover:bg-red-600 transition-colors',
                  buttonStyles.button,
                )}
                onClick={onAttack}
                disabled={isPending || !isAllowed}>
                <Icon name="swords" className="w-5 h-5" /> Attack
              </button>
              <button
                className={clsx('btn items-center gap-2 w-full', buttonStyles.secondary, buttonStyles.button)}
                onClick={onDefence}
                disabled={isPending || !isAllowed}>
                <Icon name="armor" className="w-5 h-5" /> Defence
              </button>
            </>
          )}
        </div>
      </div>
      <div className="relative basis-[40%] w-full flex flex-col">
        <TamagotchiAvatar
          color={players[1].color}
          age={players[1].dateOfBirth}
          className="grow w-full h-full "
          isActive={battle?.state !== 'WaitNextRound' && players[1].tmgId === currentPlayer}
          isWinner={battle?.state === 'WaitNextRound' && battle.currentWinner === players[1].tmgId}
          isDead={!players[1].health}
          damage={roundDamage.length > 0 ? Math.round(roundDamage[1] / 25) : 0}
        />
      </div>
    </div>
  );
};
