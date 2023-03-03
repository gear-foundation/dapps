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
  const { rivals, currentPlayer, currentPairIdx, roundDamage, battle } = useBattle();
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
    handleMessage({ StartBattle: null }, { onSuccess, onError });
  };
  const onAttack = () => {
    setIsPending(true);
    handleMessage({ MakeMove: { pair_id: currentPairIdx, tmg_move: { Attack: null } } }, { onSuccess, onError });
  };
  const onDefence = () => {
    setIsPending(true);
    handleMessage({ MakeMove: { pair_id: currentPairIdx, tmg_move: { Defence: null } } }, { onSuccess, onError });
  };

  const cnWrapper = 'relative flex flex-col';
  const cnT = 'm-auto h-full w-full max-w-full';

  return (
    <>
      {battle && (
        <div className="relative grow grid grid-cols-[40%_40%] justify-between gap-10 mt-10 xxl:mt-15">
          <div className={cnWrapper}>
            <TamagotchiAvatar
              color={rivals[0].color}
              age={rivals[0].dateOfBirth}
              className={cnT}
              isActive={battle.state !== 'WaitNextRound' && rivals[0].tmgId === currentPlayer}
              isWinner={battle.state === 'WaitNextRound' && battle.pairs[currentPairIdx].winner === rivals[0].tmgId}
              isDead={!rivals[0].health}
              damage={roundDamage ? Math.round(roundDamage[1] / 25) : 0}
              action={roundDamage && (roundDamage[3] === null ? 'Skipped' : roundDamage[3])}
              asPlayer
            />
          </div>
          <div className="absolute top-1/2 left-1/2 z-1 -translate-x-1/2 -translate-y-1/2 flex flex-col gap-3 xxl:gap-6 w-full smh:mt-5 max-w-[200px] xxl:max-w-[250px]">
            <div className="flex flex-col items-center gap-1 xxl:gap-2">
              {!battle.pairs[currentPairIdx].gameIsOver ? (
                <>
                  <p className="smh:hidden font-semibold font-sans uppercase text-[#D2D2D3] text-opacity-60 text-center tracking-[.04em]">
                    Round: {battle && battle.pairs[currentPairIdx].rounds + 1} <span className="normal-case">of</span> 5
                  </p>
                  <p className="smh:text-[26px] text-2xl leading-tight xxl:typo-h2 truncate max-w-[13ch] font-bold">
                    {currentPlayer && battle.players[currentPlayer].name}
                  </p>
                </>
              ) : (
                <p className="text-center text-2xl leading-normal xxl:typo-h2 truncate max-w-[13ch] font-bold">
                  <span className="text-primary">{battle.players[battle.pairs[currentPairIdx].winner].name}</span>
                  <br />
                  Winner
                </p>
              )}
            </div>
            <div className="space-y-2 xxl:space-y-3">
              {battle.state === 'WaitNextRound' && isAdmin && (
                <button
                  className={clsx(
                    'relative btn items-center gap-2 w-full transition-colors',
                    'before:absolute before:-inset-1 before:border before:border-primary/50 before:rounded-[90px] before:animate-wave-2',
                    'after:absolute after:-inset-2 after:border after:border-primary/30 after:rounded-[90px] after:animate-wave',
                    buttonStyles.primary,
                    buttonStyles.button,
                  )}
                  onClick={onNewRound}
                  disabled={isPending}>
                  Start New Round
                </button>
              )}
              {battle.state === 'GameIsOn' && !battle.pairs[currentPairIdx].gameIsOver && (
                <>
                  <button
                    className={clsx(
                      'btn btn--error items-center gap-2 w-full bg-error text-white transition-colors',
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
          <div className={cnWrapper}>
            <TamagotchiAvatar
              color={rivals[1].color}
              age={rivals[1].dateOfBirth}
              className={cnT}
              isActive={battle.state !== 'WaitNextRound' && rivals[1].tmgId === currentPlayer}
              isWinner={battle.state === 'WaitNextRound' && battle.pairs[currentPairIdx].winner === rivals[1].tmgId}
              isDead={!rivals[1].health}
              damage={roundDamage ? Math.round(roundDamage[2] / 25) : 0}
              action={roundDamage && (roundDamage[4] === null ? 'Skipped' : roundDamage[4])}
              reverse
              asPlayer
            />
          </div>
        </div>
      )}
    </>
  );
};
