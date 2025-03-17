import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { Modal, Segmented, Text } from '@/components';
import {
  AttackButtonIcon,
  DefenceButtonIcon,
  UltimateButtonIcon,
  UserSkullIcon,
  UserSmileIcon,
} from '@/features/game/assets/images';
import { Background, GameButton, Timer } from '@/features/game/components';
import { Character } from '@/features/game/components/character';
import { CharacterStats } from '@/features/game/components/character-stats';
import { mockCharacterView, mockCharacterView2, mockPlayer1, mockPlayer2 } from '@/features/game/consts';
import { characterAppearanceStorage, characterStatsStorage } from '@/features/game/store';

import styles from './onboarding.module.scss';

const steps = [
  {
    title: 'This is your character',
    children: (
      <Text>
        You will attack and defend against the opponent. The goal is to defeat as many opponents as possible. The player
        who defeats all others wins the game.
      </Text>
    ),
  },
  {
    title: 'How to play',
    children: (
      <>
        <Text>
          Press <span className={styles.attack}>Attack</span> to deal damage to your opponent. The more attack points
          you have, the harder you&apos;ll hit.
        </Text>
        <Text>
          <span className={styles.reflect}>Reflect</span> returns a part of the enemy&apos;s attack damage equal to your
          defence. It can only be used once every few turns.
        </Text>
        <Text>
          <span className={styles.ultimate}>Ultimate</span> doubles your attack damage and is also active once every few
          turns.
        </Text>
      </>
    ),
  },
  {
    title: 'Players stats overview',
    children: (
      <>
        <Text>
          <span className={styles.attack}>Attack</span> points show the strength of the damage you deal to the opponent.
        </Text>
        <Text>
          <span className={styles.reflect}>Defence</span> indicates how efficiently opponent&apos;s attack can be
          reflected.
        </Text>
        <Text>
          <span className={styles.dodge}>Dodge</span> chance: the probability of fully evading the opponent’s attack.
        </Text>
        <Text>
          You can also track the damage dealt to you by the opponent, which will affect your{' '}
          <span className={styles.ultimate}>Health</span> stat.
        </Text>
        <Text>If it&apos;s your turn, your character&apos;s image will be highlighted in green.</Text>
      </>
    ),
  },
  {
    title: 'Players list and active battles',
    children: (
      <Text>
        At the bottom of the screen, you can view the list of active and eliminated players, as well as active battles
        with all the moves you and your opponent made.
      </Text>
    ),
  },
];

export function Onboarding() {
  const navigate = useNavigate();
  const characterView = characterAppearanceStorage.get() || mockCharacterView;
  const characterStats = characterStatsStorage.get();

  useEffect(() => {
    window.scrollTo({ top: 0 });
  }, []);

  const onClose = () => {
    navigate(-1);
  };

  const [step, setStep] = useState(0);

  const timeLeft = 12000;

  const segmentedOptions = [
    {
      label: (
        <div className={styles.players}>
          <span>Players:</span>
          <div>
            12 <UserSmileIcon />
          </div>
          <div>
            36 <UserSkullIcon />
          </div>
        </div>
      ),
      value: 'players',
    },
    {
      label: 'Active Battles',
      value: 'battles',
    },
  ];

  const { title, children } = steps[step];

  return (
    <>
      <Background>
        <CharacterStats
          align="left"
          {...mockPlayer1}
          {...characterStats}
          characterView={characterView}
          className={clsx(step === 2 && styles.highlighted)}
          isActive
        />
        <div className={clsx(styles.character, styles.left)}>
          <Character {...characterView} />
        </div>

        {<Timer remainingTime={timeLeft} shouldGoOn={false} />}

        <CharacterStats align="right" {...mockPlayer2} characterView={mockCharacterView2} />
        <div className={clsx(styles.character, styles.right)}>
          <Character {...mockCharacterView2} />
        </div>

        <Modal
          title={title}
          className={styles.modal}
          modalClassName={styles.backdrop}
          onClose={onClose}
          size="sm"
          closeOnBackdrop={false}>
          {children}
          <div className={styles.modalButtons}>
            {step !== 0 && <Button text="Back" color="grey" onClick={() => setStep(step - 1)} />}
            {step === steps.length - 1 ? (
              <Button text={'Got it!'} onClick={onClose} />
            ) : (
              <Button text={'Continue'} onClick={() => setStep(step + 1)} />
            )}
            <Text size="md">
              {step + 1}/{steps.length}
            </Text>
          </div>

          {step === 1 && (
            <div className={styles.gameButtons}>
              <GameButton color="red" text="Attack" icon={<AttackButtonIcon />} />
              <GameButton color="green" text="Reflect" icon={<DefenceButtonIcon />} />
              <GameButton color="cyan" text="Ultimate" icon={<UltimateButtonIcon />} />
            </div>
          )}
          {step === 3 && (
            <Segmented className={styles.segmented} options={segmentedOptions} value="battles" onChange={() => {}} />
          )}
        </Modal>
      </Background>

      <div className={styles['blur-backdrop']}>
        {step === 0 && (
          <div className={styles.characterCircle}>
            <Background>
              <div className={clsx(styles.character, styles.left)}>
                <Character {...characterView} />
              </div>
            </Background>
          </div>
        )}
      </div>
    </>
  );
}
