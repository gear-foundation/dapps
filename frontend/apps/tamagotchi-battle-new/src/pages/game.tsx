import { useNavigate } from 'react-router-dom';

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
import { Timer } from '@/features/game/components/timer';
import { GameButton } from '@/features/game/components/game-button';
import { useState } from 'react';
import { Switcher } from '@dapps-frontend/ui';
import { Segmented } from '@/components/ui/segmented/segmented';

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
  return (
    <>
      <Background>
        <CharacterStats align="left" />
        <div className={clsx(styles.character, styles.left)}>
          <Character />
        </div>

        <Timer remainingTime={50000} shouldGoOn={true} />

        <CharacterStats align="right" />
        <div className={clsx(styles.character, styles.right)}>
          <Character />
        </div>

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
        <Segmented
          className={styles.segmented}
          options={segmentedOptions}
          value={selectedTab}
          onChange={(value) => setSelectedTab(value as Tabs)}
        />
      </Background>
    </>
  );
}
