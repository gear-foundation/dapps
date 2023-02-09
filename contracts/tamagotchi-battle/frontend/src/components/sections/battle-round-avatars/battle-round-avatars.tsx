import { TamagotchiAvatar } from '../../tamagotchi/tamagotchi-avatar';
import { useApp, useBattle } from '../../../app/context';
import { getTamagotchiAgeDiff } from '../../../app/utils/get-tamagotchi-age';
import clsx from 'clsx';
import { buttonStyles } from '@gear-js/ui';
import { BattleStateResponse } from '../../../app/types/battles';
import { Icon } from '../../ui/icon';
import { useBattleMessage } from '../../../app/hooks/use-battle';
import { useEffect, useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';

export const BattleRoundAvatars = ({ battle }: { battle: BattleStateResponse }) => {
  const { isPending, setIsPending } = useApp();
  const { account } = useAccount();
  const { players, currentPlayer, roundDamage } = useBattle();
  const [isAllowed, setIsAllowed] = useState<boolean>(false);
  const handleMessage = useBattleMessage();

  useEffect(() => {
    if (battle && account && currentPlayer) {
      setIsAllowed(battle.players[currentPlayer].owner === account.decodedAddress);
    }
  }, [account, battle, currentPlayer]);

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
    <div className="relative grow flex justify-between gap-10 mt-10 xl:mt-15">
      <div className="relative basis-[40%] w-full flex flex-col">
        <TamagotchiAvatar
          inBattle
          color={players[0].color}
          age={getTamagotchiAgeDiff(players[0].dateOfBirth)}
          hasItem={[]}
          energy={players[0].health}
          className="grow w-full h-full "
          isActive={battle.state !== 'WaitNextRound' && players[0].tmgId === currentPlayer}
          isWinner={battle.state === 'WaitNextRound' && battle.currentWinner === players[0].tmgId}
          isDead={!players[0].health}
          damage={roundDamage && roundDamage[0]}
        />
      </div>
      <div className="absolute top-1/2 left-1/2 z-1 -translate-x-1/2 -translate-y-1/2 flex flex-col gap-6 w-full max-w-[250px]">
        <div className="flex flex-col items-center">
          <p className="text-2xl leading-normal xl:typo-h2 truncate max-w-[9ch]">
            {currentPlayer && battle.players[currentPlayer].name}
          </p>
        </div>
        <div className="space-y-3">
          {battle.state === 'WaitNextRound' && (
            <button
              className={clsx(
                'btn items-center gap-2 w-full transition-colors',
                buttonStyles.primary,
                buttonStyles.button,
              )}
              onClick={onNewRound}
              disabled={isPending || !isAllowed}>
              Start New Round
            </button>
          )}
          {battle.state === 'GameIsOn' && (
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
          inBattle
          color={players[1].color}
          age={getTamagotchiAgeDiff(players[1].dateOfBirth)}
          hasItem={[]}
          energy={players[1].health}
          className="grow w-full h-full "
          isActive={battle.state !== 'WaitNextRound' && players[1].tmgId === currentPlayer}
          isWinner={battle.state === 'WaitNextRound' && battle.currentWinner === players[1].tmgId}
          isDead={!players[1].health}
          damage={roundDamage && roundDamage[1]}
        />
      </div>
    </div>
  );
};
