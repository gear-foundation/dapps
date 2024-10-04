import { useNavigate } from 'react-router-dom';
import { Button } from '@gear-js/vara-ui';

import { Background } from '@/features/game/components';
import { Character } from '@/features/game/components/character';
import { CharacterStats } from '@/features/game/components/character-stats';
import {
  AttackButtonIcon,
  DefenseButtonIcon,
  UltimateButtonIcon,
  UserSkullIcon,
  UserSmileIcon,
} from '@/features/game/assets/images';

import styles from './game.module.scss';
import clsx from 'clsx';
import { BattleHistoryCard, GameButton, GameOverCard, Timer } from '@/features/game/components';
import { useState } from 'react';
import { Segmented } from '@/components';
import { List } from '@/features/game/components/list/list';
import { ExitIcon } from '@/features/wallet/assets';
import { BattleCard } from '@/features/game/components/battle-card';
import { mockPlayer1, mockPlayer2 } from '@/features/game/mock';
import { PlayersList } from '@/features/game/components/playersList';
import { PlayerStatus } from '@/features/game/types';
import { mockCharacterView } from '@/features/game/consts';

type Tabs = 'players' | 'history';

export default function GamePage() {
  const navigate = useNavigate();

  const tournamentName = 'Tournament name';
  const isAdmin = true;

  const [tappedButton, setTappedButton] = useState<'attack' | 'reflect' | 'ultimate' | null>(null);
  const [selectedTab, setSelectedTab] = useState<Tabs>('players');

  const onAttackClick = () => {
    setTappedButton('attack');
  };
  const onReflectClick = () => {
    setTappedButton('reflect');
  };
  const onUltimateClick = () => {
    setTappedButton('ultimate');
  };

  const [isShowTurnEndCard, setIsShowTurnEndCard] = useState(true);
  const isTurnEnd = true;
  const isBattleEnd = mockPlayer1.currentHealth === 0 || mockPlayer2.currentHealth === 0;

  const isPlayerDefeated = true;
  const isGameOver = isPlayerDefeated;
  const isTournamentOver = false;

  const timeLeft = 50000;
  const prizeCount = 100;

  const showOtherBattles = isPlayerDefeated;
  const showPlayersList = isPlayerDefeated && selectedTab === 'players';

  const playersListItems = [
    { name: 'Sandeeps', status: 'defeated' as PlayerStatus },
    { name: 'CodeWithSomya', status: 'alive' as PlayerStatus },
    { name: 'Shivam98', status: 'defeated' as PlayerStatus },
  ];

  const alivePlayersCount = playersListItems.reduce((acc, { status }) => (status === 'alive' ? acc + 1 : acc), 0);

  const segmentedOptions = [
    {
      label: (
        <div className={styles.players}>
          <span>Players:</span>
          <div>
            {alivePlayersCount} <UserSmileIcon />
          </div>
          <div>
            {playersListItems.length - alivePlayersCount} <UserSkullIcon />
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

  return (
    <>
      <Background>
        <CharacterStats align="left" {...mockPlayer1} characterView={mockCharacterView} />
        <div className={clsx(styles.character, styles.left)}>
          <Character {...mockCharacterView} />
        </div>

        {!isTurnEnd && <Timer remainingTime={timeLeft} shouldGoOn={true} />}

        <CharacterStats align="right" {...mockPlayer2} characterView={mockCharacterView} />
        <div className={clsx(styles.character, styles.right)}>
          <Character {...mockCharacterView} />
        </div>

        {!isBattleEnd && (
          <div className={styles.buttons}>
            <GameButton
              onClick={onAttackClick}
              color="red"
              text="Attack"
              icon={<AttackButtonIcon />}
              pending={tappedButton === 'attack'}
              // turnsBlocked={1}
            />
            <GameButton
              onClick={onReflectClick}
              color="green"
              text="Reflect"
              icon={<DefenseButtonIcon />}
              pending={tappedButton === 'reflect'}
            />
            <GameButton
              onClick={onUltimateClick}
              color="cyan"
              text="Ultimate"
              icon={<UltimateButtonIcon />}
              pending={tappedButton === 'ultimate'}
            />
          </div>
        )}
        {isTurnEnd && !isBattleEnd && !isGameOver && (
          <Button color="primary" className={styles.nextButton} text={`Next turn (${timeLeft / 1000})`} />
        )}
        {isBattleEnd && !isGameOver && (
          <Button color="primary" className={styles.nextButton} text={`Start next battle`} />
        )}
        {isTurnEnd && isShowTurnEndCard && !isGameOver && (
          <div className={clsx(styles.historyItem, styles.endTurnHistory)}>
            <BattleHistoryCard {...mockPlayer1} />
            <BattleHistoryCard {...mockPlayer2} align="right" onClose={() => setIsShowTurnEndCard(false)} />
          </div>
        )}
        {/* {isGameOver && (
          <GameOverCard
            className={styles.gameOver}
            prizeCount={prizeCount}
            isTournamentOver={isTournamentOver}
            result="lose"
            player1name={mockPlayer1.name}
            player2name={mockPlayer2.name}
          />
        )} */}
        <Segmented
          className={styles.segmented}
          options={segmentedOptions}
          value={selectedTab}
          onChange={(value) => setSelectedTab(value as Tabs)}
        />
        {isAdmin ? (
          <Button text="Cancel tournament" size="small" className={clsx(styles.cancelTournament, styles.redButton)} />
        ) : (
          <Button text="Exit" icon={ExitIcon} color="transparent" className={styles.exit} />
        )}

        {showPlayersList && (
          <PlayersList
            bid={prizeCount}
            items={playersListItems}
            className={styles.list}
            tournamentName={tournamentName}
          />
        )}

        {selectedTab === 'history' && (
          <>
            {showOtherBattles ? (
              <List
                className={styles.list}
                maxLength={7}
                items={[
                  <div className={styles.historyItem}>
                    <BattleCard {...mockPlayer1} characterView={mockCharacterView} winsCount={20} />
                    <BattleCard {...mockPlayer2} characterView={mockCharacterView} align="right" />
                  </div>,
                  <div className={styles.historyItem}>
                    <BattleCard {...mockPlayer1} characterView={mockCharacterView} />
                    <BattleCard {...mockPlayer2} characterView={mockCharacterView} align="right" winsCount={10} />
                  </div>,
                ]}
              />
            ) : (
              <List
                className={styles.list}
                maxLength={6}
                items={[
                  <div className={styles.historyItem}>
                    <BattleHistoryCard {...mockPlayer1} />
                    <BattleHistoryCard {...mockPlayer2} align="right" />
                  </div>,
                  <div className={styles.historyItem}>
                    <BattleHistoryCard {...mockPlayer1} />
                    <BattleHistoryCard {...mockPlayer2} align="right" />
                  </div>,
                ]}
              />
            )}
          </>
        )}
      </Background>
    </>
  );
}
