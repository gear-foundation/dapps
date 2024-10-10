import clsx from 'clsx';
import { useNavigate } from 'react-router-dom';
import { Button } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';

import { Background } from '@/features/game/components';
import { Character } from '@/features/game/components/character';
import { CharacterStats } from '@/features/game/components/character-stats';
import {
  AttackButtonIcon,
  DefenceButtonIcon,
  UltimateButtonIcon,
  UserSkullIcon,
  UserSmileIcon,
} from '@/features/game/assets/images';

import { BattleHistoryCard, GameButton, GameOverCard, Timer } from '@/features/game/components';
import { useEffect, useState } from 'react';
import { Loader, Segmented } from '@/components';
import { List } from '@/features/game/components/list/list';
import { ExitIcon } from '@/features/wallet/assets';
import { BattleCard } from '@/features/game/components/battle-card';
import { mockPlayer1, mockPlayer2 } from '@/features/game/mock';
import { PlayersList } from '@/features/game/components/playersList';
import { PlayerStatus } from '@/features/game/types';
import { SphereAnimation, FireballCanvas } from '@/features/game/components/animations';
import {
  Player,
  useCancelTournamentMessage,
  useConfigQuery,
  useMakeMoveMessage,
  useMyBattleQuery,
  useStartNextFightMessage,
} from '@/app/utils';
import { ROUTES } from '@/app/consts';
import styles from './game.module.scss';

type Tabs = 'players' | 'history';

export default function GamePage() {
  const navigate = useNavigate();
  const { account } = useAccount();
  const { battleState, isFetching } = useMyBattleQuery();
  const { config } = useConfigQuery();
  const { cancelTournamentMessage } = useCancelTournamentMessage();
  const { startNextFightMessage } = useStartNextFightMessage();
  const { makeMoveMessage } = useMakeMoveMessage();

  const [tappedButton, setTappedButton] = useState<'attack' | 'reflect' | 'ultimate' | null>(null);
  const [selectedTab, setSelectedTab] = useState<Tabs>('players');
  const [isShowTurnEndCard, setIsShowTurnEndCard] = useState(false);
  const [isShowNextTurnButton, setIsShowNextTurnButton] = useState(false);

  const { pairs, players_to_pairs } = battleState || {};
  const pairId = players_to_pairs?.find(([address]) => account.decodedAddress === address)?.[1];
  const pair = pairs?.find(([number]) => pairId === number)?.[1];

  useEffect(() => {
    if (!isFetching && !battleState) {
      navigate(ROUTES.HOME);
    }
  }, [isFetching, battleState]);

  useEffect(() => {
    if (pair?.round) {
      setIsShowNextTurnButton(true);
      setIsShowTurnEndCard(true);
      setTappedButton(null);
    }
  }, [pair?.round]);

  if (!battleState || !config) {
    return <Loader />;
  }

  const { battle_name, admin, state, participants, defeated_participants } = battleState;
  console.log('🚀 ~ GamePage ~ battleState:', battleState);

  console.log('🚀 ~ GamePage ~ pairs:', pairs);
  const { player_1, player_2, round_start_time } = pair || {};

  const allParticipants = [...participants, ...defeated_participants];

  const participantsMap = allParticipants.reduce(
    (acc, [key, player]) => {
      acc[key] = player;
      return acc;
    },
    {} as Record<string, Player>,
  );

  const opponentsAddress = account.decodedAddress === player_1 ? player_2 : player_1;

  const me = participantsMap[account.decodedAddress];
  const opponent = opponentsAddress ? participantsMap[opponentsAddress] : null;

  const isAdmin = account.decodedAddress === admin;

  const onAttackClick = () => {
    setTappedButton('attack');
    makeMoveMessage('attack', { onError: () => setTappedButton(null) });
  };
  const onReflectClick = () => {
    setTappedButton('reflect');
    makeMoveMessage('reflect', { onError: () => setTappedButton(null) });
  };
  const onUltimateClick = () => {
    setTappedButton('ultimate');
    makeMoveMessage('ultimate', { onError: () => setTappedButton(null) });
  };

  const isTurnEnd = tappedButton !== null;
  const isBattleEnd = mockPlayer1.health === 0 || mockPlayer2.health === 0;

  const isPlayerDefeated = true;
  const isTournamentOver = 'gameIsOver' in state;
  const tournamentWinnerName = isTournamentOver ? participantsMap[state.gameIsOver.winner].user_name : null;

  const resultStatus = isTournamentOver && state.gameIsOver.winner === account.decodedAddress ? 'win' : 'lose';
  // const isTournamentOver = false;

  const roundDuration = config.time_for_move_in_blocks * config.block_duration_ms;
  const timeLeft = round_start_time ? Number(round_start_time) + roundDuration - Date.now() : null;
  const prizeCount = 100;

  const showOtherBattles = isPlayerDefeated;
  const showPlayersList = isPlayerDefeated && selectedTab === 'players';

  const alivePlayersListItems = participants.map(([_, { user_name }]) => ({
    name: user_name,
    status: 'alive' as PlayerStatus,
  }));
  const defeatedPlayersListItems = defeated_participants.map(([_, { user_name }]) => ({
    name: user_name,
    status: 'defeated' as PlayerStatus,
  }));

  const playersListItems = [...alivePlayersListItems, ...defeatedPlayersListItems];

  const segmentedOptions = [
    {
      label: (
        <div className={styles.players}>
          <span>Players:</span>
          <div>
            {participants.length} <UserSmileIcon />
          </div>
          <div>
            {defeated_participants.length} <UserSkullIcon />
          </div>
        </div>
      ),
      value: 'players',
    },
    {
      label: 'Battle History ',
      value: 'history',
    },
  ];

  const onCancelTournament = () => {
    cancelTournamentMessage({ onSuccess: () => navigate(ROUTES.HOME) });
  };

  return (
    <>
      <Background>
        <CharacterStats
          align="left"
          {...mockPlayer1}
          characterView={me.appearance}
          name={me.user_name}
          {...me.player_settings}
        />
        <div className={clsx(styles.character, styles.left)}>
          <Character {...me.appearance} />
          <SphereAnimation className={styles.fireSphere} type="attack" />
        </div>

        <FireballCanvas />
        {!isTurnEnd && opponent && !isShowNextTurnButton && <Timer remainingTime={timeLeft} shouldGoOn={true} />}

        {/* ! TODO if no opponent */}
        {opponent && (
          <>
            <CharacterStats
              align="right"
              {...mockPlayer2}
              characterView={opponent.appearance}
              name={opponent.user_name}
              {...opponent.player_settings}
            />
            <div className={clsx(styles.character, styles.right)}>
              <Character {...opponent.appearance} />
              <SphereAnimation className={styles.fireSphere} type="ultimate" />
            </div>
          </>
        )}

        {!isBattleEnd && !isTournamentOver && !isShowNextTurnButton && (
          <div className={styles.buttons}>
            <GameButton
              onClick={onAttackClick}
              color="red"
              text="Attack"
              icon={<AttackButtonIcon />}
              pending={tappedButton === 'attack'}
            />
            <GameButton
              onClick={onReflectClick}
              color="green"
              text="Reflect"
              icon={<DefenceButtonIcon />}
              pending={tappedButton === 'reflect'}
              turnsBlocked={me.reflect_reload}
            />
            <GameButton
              onClick={onUltimateClick}
              color="cyan"
              text="Ultimate"
              icon={<UltimateButtonIcon />}
              pending={tappedButton === 'ultimate'}
              turnsBlocked={me.ultimate_reload}
            />
          </div>
        )}
        {isBattleEnd && !isTournamentOver && (
          <Button
            color="primary"
            className={styles.nextButton}
            text={`Start next battle`}
            onClick={() => startNextFightMessage()}
          />
        )}
        {isShowTurnEndCard && !isTournamentOver && (
          <div className={clsx(styles.historyItem, styles.endTurnHistory)}>
            <BattleHistoryCard {...mockPlayer1} />
            <BattleHistoryCard {...mockPlayer2} align="right" onClose={() => setIsShowTurnEndCard(false)} />
          </div>
        )}
        {isShowNextTurnButton && !isBattleEnd && !isTournamentOver && timeLeft && (
          <Button
            color="primary"
            className={styles.nextButton}
            text={`Next turn (${timeLeft / 1000})`}
            onClick={() => setIsShowNextTurnButton(false)}
          />
        )}
        {isTournamentOver && tournamentWinnerName && (
          <GameOverCard
            className={styles.gameOver}
            prizeCount={prizeCount}
            isTournamentOver={isTournamentOver}
            result={resultStatus}
            player1name={tournamentWinnerName}
            player2name={mockPlayer2.name}
          />
        )}
        <Segmented
          className={styles.segmented}
          options={segmentedOptions}
          value={selectedTab}
          onChange={(value) => setSelectedTab(value as Tabs)}
        />
        {isAdmin ? (
          <Button
            text="Cancel tournament"
            size="small"
            className={clsx(styles.cancelTournament, styles.redButton)}
            onClick={onCancelTournament}
          />
        ) : (
          <Button text="Exit" icon={ExitIcon} color="transparent" className={styles.exit} />
        )}

        {showPlayersList && (
          <PlayersList
            bid={Number(battleState.bid)}
            items={playersListItems}
            className={styles.list}
            tournamentName={battle_name}
          />
        )}

        {selectedTab === 'history' && (
          <>
            {showOtherBattles ? (
              <List
                className={styles.list}
                maxLength={7}
                items={battleState.pairs.map(([key, { player_1, player_2 }]) => {
                  const player1 = participantsMap[player_1];
                  const player2 = participantsMap[player_2];

                  return (
                    <div key={key} className={styles.historyItem}>
                      <BattleCard
                        {...player1.player_settings}
                        name={player1.user_name}
                        characterView={player1.appearance}
                        winsCount={player1.number_of_victories}
                      />
                      <BattleCard
                        {...player2.player_settings}
                        name={player2.user_name}
                        characterView={player2.appearance}
                        winsCount={player2.number_of_victories}
                        align="right"
                      />
                    </div>
                  );
                })}
              />
            ) : (
              <List
                className={styles.list}
                maxLength={6}
                items={battleState.pairs.map(([key, { player_1, player_2 }]) => {
                  const player1 = participantsMap[player_1];
                  const player2 = participantsMap[player_2];

                  return (
                    <div key={key} className={styles.historyItem}>
                      <BattleHistoryCard {...mockPlayer1} />
                      <BattleHistoryCard {...mockPlayer2} align="right" />
                    </div>
                  );
                })}
              />
            )}
          </>
        )}
      </Background>
    </>
  );
}
