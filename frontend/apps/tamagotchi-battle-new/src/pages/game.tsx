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
import { PlayerState } from '@/features/game/types';

const mockPlayer1: PlayerState = {
  name: 'Player name 1',
  currentHealth: 40,
  attack: 30,
  deffence: 10,
  dodge: 30,
  playerId: 1,
  action: 'attack',
  isDodged: true,
  recivedDamage: 13,
};

const mockPlayer2: PlayerState = {
  name: 'Player name 2',
  currentHealth: 0,
  attack: 10,
  deffence: 13,
  dodge: 5,
  playerId: 2,
  action: 'reflect',
  isDodged: false,
  recivedDamage: 0,
};

const segmentedOptions = [
  {
    label: (
      <div className={styles.players}>
        <span>Players:</span>
        <div>
          21 <UserSmileIcon />
        </div>
        <div>
          11 <UserSkullIcon />
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

  const isGameOver = false;
  const isTournamentOver = false;

  const timeLeft = 50000;
  const prizeCount = 100;

  return (
    <>
      <Background>
        <CharacterStats align="left" {...mockPlayer1} />
        <div className={clsx(styles.character, styles.left)}>
          <Character />
        </div>

        {!isTurnEnd && <Timer remainingTime={timeLeft} shouldGoOn={true} />}

        <CharacterStats align="right" {...mockPlayer2} />
        <div className={clsx(styles.character, styles.right)}>
          <Character />
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

        {isGameOver && (
          <GameOverCard
            className={styles.gameOver}
            prizeCount={prizeCount}
            isTournamentOver={isTournamentOver}
            result="lose"
            player1name={mockPlayer1.name}
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
          <Button text="Cancel tournament" size="small" className={clsx(styles.cancelTournament, styles.redButton)} />
        ) : (
          <Button text="Exit" icon={ExitIcon} color="transparent" className={styles.exit} />
        )}
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

        <List className={styles.list} maxLength={7} items={[]} />
      </Background>
    </>
  );
}
