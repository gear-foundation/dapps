import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';
import { useAtom, useAtomValue, useSetAtom } from 'jotai';
import { useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import {
  Move,
  useCancelTournamentMessage,
  useConfigQuery,
  useExitGameMessage,
  useMakeMoveMessage,
  useMyBattleQuery,
  useStartNextFightMessage,
} from '@/app/utils';
import { Loader, Modal } from '@/components';
import { AttackButtonIcon, DefenceButtonIcon, ExitIcon, UltimateButtonIcon } from '@/features/game/assets/images';
import {
  Background,
  BattleTabs,
  BattleHistorySinc,
  Character,
  CharacterStats,
  BattleHistoryCard,
  GameButton,
  GameOverCard,
  Timer,
  SphereAnimation,
  FireballCanvas,
  GameSpinner,
} from '@/features/game/components';
import { useParticipants, usePending } from '@/features/game/hooks';
import {
  battleHistoryAtom,
  battleHistoryStorage,
  currentPlayersAtom,
  currentPlayersStorage,
  otherPairBattleWatchAtom,
} from '@/features/game/store';

import styles from './game.module.scss';

export default function GamePage() {
  const navigate = useNavigate();
  const { account } = useAccount();
  const { battleState, isFetching } = useMyBattleQuery();
  const { pending } = usePending();
  const { config } = useConfigQuery();
  const { cancelTournamentMessage } = useCancelTournamentMessage();
  const { startNextFightMessage } = useStartNextFightMessage();
  const { makeMoveMessage } = useMakeMoveMessage();
  const { exitGameMessage } = useExitGameMessage();
  const tabsRef = useRef<HTMLDivElement>(null);

  const [isOpenCancelTournamentModal, setIsOpenCancelTournamentModal] = useState(false);

  const [tappedButton, setTappedButton] = useState<Move | null>(null);
  const [showAnimation, setShowAnimation] = useState(false);

  const battleHistory = useAtomValue(battleHistoryAtom);
  const lastTurnHistory = battleHistory?.[0];

  const [isShowTurnEndCard, setIsShowTurnEndCard] = useState(false);

  const [otherPairBattleWatch] = useAtom(otherPairBattleWatchAtom);
  const isShowOtherBattle = Boolean(battleState?.pairs.some(([pairId]) => pairId === otherPairBattleWatch));

  const { allParticipants, isAlive, hasPlayer, hasOpponent, participantsMap, pair, currentPlayers } =
    useParticipants(battleState);

  const { player, opponent } = currentPlayers || {};

  const { admin, state, waiting_player, bid } = battleState || {};

  useEffect(() => {
    if (!isFetching && !battleState) {
      navigate(ROUTES.HOME);
    }
  }, [isFetching, battleState, navigate]);

  const turnEndCallback = () => {
    setIsShowTurnEndCard(true);
    setTappedButton(null);
    setShowAnimation(true);
    setTimeout(() => {
      setShowAnimation(false);
    }, 3000);
  };

  const setBattleHistory = useSetAtom(battleHistoryAtom);
  const setCurrentPlayers = useSetAtom(currentPlayersAtom);

  if (!battleState || !config || !state || !account) {
    return <Loader />;
  }

  const showStartNextBattle = !hasOpponent && waiting_player?.[0] !== account.decodedAddress && isAlive;
  const showWaitingForOpponent = waiting_player?.[0] === account.decodedAddress;
  const isAdmin = account.decodedAddress === admin;
  const isTournamentOver = 'gameIsOver' in state;
  const isCurrentDraw =
    !isTournamentOver && battleHistory?.[0].player.health === 0 && battleHistory?.[0].opponent.health === 0;

  const onAttackClick = () => {
    setTappedButton('Attack');
    makeMoveMessage('Attack', { onError: () => setTappedButton(null) });
  };
  const onReflectClick = () => {
    setTappedButton('Reflect');
    makeMoveMessage('Reflect', { onError: () => setTappedButton(null) });
  };
  const onUltimateClick = () => {
    setTappedButton('Ultimate');
    makeMoveMessage('Ultimate', { onError: () => setTappedButton(null) });
  };

  const { round_start_time } = pair || {};
  const roundDuration = config.time_for_move_in_blocks * config.block_duration_ms;
  const timeLeft = round_start_time ? Number(round_start_time) + roundDuration - Date.now() : null;

  const onCancelTournament = () => {
    cancelTournamentMessage({ onSuccess: () => navigate(ROUTES.HOME) });
  };

  const onExitGame = () => {
    exitGameMessage({
      onSuccess: () => navigate(ROUTES.HOME),
    });
  };

  return (
    <>
      <Background>
        {player && (
          <>
            <CharacterStats
              align="left"
              characterView={player.appearance}
              name={player.user_name}
              {...player.player_settings}
            />
            {(player.player_settings.health !== 0 || isCurrentDraw) && (
              <div className={clsx(styles.character, styles.left)}>
                <Character {...player.appearance} />

                <SphereAnimation
                  className={styles.fireSphere}
                  type={tappedButton || (showAnimation ? lastTurnHistory?.player.action : undefined)}
                />
              </div>
            )}
          </>
        )}

        {hasPlayer && hasOpponent && !showAnimation && (
          <Timer remainingTime={timeLeft} isYourTurn={tappedButton === null && !isShowOtherBattle} />
        )}

        {opponent && (
          <>
            <CharacterStats
              align="right"
              characterView={opponent.appearance}
              name={opponent.user_name}
              {...opponent.player_settings}
            />
            {(opponent.player_settings.health !== 0 || isCurrentDraw) && (
              <div className={clsx(styles.character, styles.right)}>
                <Character {...opponent.appearance} />

                {showAnimation && (
                  <SphereAnimation className={styles.fireSphere} type={lastTurnHistory?.opponent.action} />
                )}
              </div>
            )}
          </>
        )}

        {lastTurnHistory && showAnimation && <FireballCanvas lastTurnHistory={lastTurnHistory} />}

        {showWaitingForOpponent ||
          (hasOpponent && hasPlayer && player && opponent && !isTournamentOver && !isShowOtherBattle && (
            <div className={styles.buttons}>
              <GameButton
                onClick={onAttackClick}
                color="red"
                text="Attack"
                icon={<AttackButtonIcon />}
                pending={tappedButton === 'Attack' || pending}
                disabled={showWaitingForOpponent || !!(tappedButton && tappedButton !== 'Attack')}
              />
              <GameButton
                onClick={onReflectClick}
                color="green"
                text="Reflect"
                icon={<DefenceButtonIcon />}
                pending={tappedButton === 'Reflect' || pending}
                turnsBlocked={player.reflect_reload}
                disabled={showWaitingForOpponent || !!(tappedButton && tappedButton !== 'Reflect')}
              />
              <GameButton
                onClick={onUltimateClick}
                color="cyan"
                text="Ultimate"
                icon={<UltimateButtonIcon />}
                pending={tappedButton === 'Ultimate' || pending}
                turnsBlocked={player.ultimate_reload}
                disabled={showWaitingForOpponent || !!(tappedButton && tappedButton !== 'Ultimate')}
              />
            </div>
          ))}

        {showStartNextBattle && !isTournamentOver && !isShowOtherBattle && (
          <Button
            color="primary"
            className={styles.nextButton}
            text={`Start next battle`}
            onClick={() => {
              setBattleHistory(null);
              battleHistoryStorage.set(null);
              setCurrentPlayers(null);
              currentPlayersStorage.set(null);
              startNextFightMessage();
            }}
            disabled={pending}
          />
        )}

        {showWaitingForOpponent && <GameSpinner text="Please wait for your opponent" />}

        {player && opponent && pair && (
          <BattleHistorySinc player={player} opponent={opponent} turnEndCallback={turnEndCallback} pair={pair} />
        )}

        {isShowTurnEndCard &&
          lastTurnHistory &&
          player &&
          opponent &&
          !isTournamentOver &&
          !isCurrentDraw &&
          isAlive && (
            <div className={clsx(styles.historyItem, styles.endTurnHistory)}>
              <BattleHistoryCard {...player.player_settings} {...lastTurnHistory.player} name={player.user_name} />
              <BattleHistoryCard
                {...opponent.player_settings}
                {...lastTurnHistory.opponent}
                name={opponent.user_name}
                align="right"
                onClose={() => setIsShowTurnEndCard(false)}
              />
            </div>
          )}

        <GameOverCard
          className={styles.gameOver}
          bid={Number(bid)}
          totalParticipants={allParticipants.length}
          state={state}
          participantsMap={participantsMap}
          isAlive={isAlive}
          isShowOtherBattle={isShowOtherBattle}
          onScrollToHistoryClick={() => tabsRef.current?.scrollIntoView({ behavior: 'smooth' })}
        />

        {isAdmin ? (
          <Button
            text="Cancel tournament"
            size="small"
            className={clsx(styles.cancelTournament, styles.redButton, !isAlive && styles.defeated)}
            onClick={() => (isTournamentOver ? onCancelTournament() : setIsOpenCancelTournamentModal(true))}
            disabled={pending}
          />
        ) : (
          <Button
            text="Exit"
            size="small"
            icon={ExitIcon}
            className={clsx(styles.exit, styles.redButton, !isAlive && styles.defeated)}
            onClick={onExitGame}
            disabled={pending}
          />
        )}

        <BattleTabs
          battleState={battleState}
          participantsMap={participantsMap}
          isAlive={isAlive}
          tabsRef={tabsRef}
          isSpectating={isShowOtherBattle}
        />

        {isOpenCancelTournamentModal && (
          <Modal
            title="Sure you want to end the game?"
            description={`This action cannot be undone. The game will be concluded, and all players will exit the gaming room. ${
              !isTournamentOver ? 'Any entry fees will be refunded to all players.' : ''
            }`}
            className={styles.cancelTournamentModal}
            onClose={() => setIsOpenCancelTournamentModal(false)}
            buttons={
              <>
                <Button color="grey" text="End tournament" onClick={onCancelTournament} disabled={pending} />
                <Button
                  color="primary"
                  text="Continue tournament"
                  onClick={() => setIsOpenCancelTournamentModal(false)}
                  disabled={pending}
                />
              </>
            }
          />
        )}
      </Background>
    </>
  );
}
