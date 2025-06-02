import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';
import { useState } from 'react';
import { NavigationType, useNavigate, useNavigationType } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
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

const STEPS = [
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
          <span className={styles.dodge}>Dodge</span> chance: the probability of fully evading the opponent&apos;s
          attack.
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
] as const;

const SEGMENTED_OPTIONS = [
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

export function Onboarding() {
  const navigate = useNavigate();
  const navigationType = useNavigationType();

  const characterView = characterAppearanceStorage.get() || mockCharacterView;
  const characterStats = characterStatsStorage.get();

  const goBack = () => {
    const hasPreviousPath = navigationType !== NavigationType.Pop;

    return hasPreviousPath ? navigate(-1) : navigate(ROUTES.HOME);
  };

  const [step, setStep] = useState(0);

  const { title, children } = STEPS[step];
  const isFirstStep = step === 0;
  const isLastStep = step === STEPS.length - 1;

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

        <Timer remainingTime={12000} shouldGoOn={false} />

        <CharacterStats align="right" {...mockPlayer2} characterView={mockCharacterView2} />

        <div className={clsx(styles.character, styles.right)}>
          <Character {...mockCharacterView2} />
        </div>

        <Modal
          title={title}
          className={styles.modal}
          modalClassName={styles.backdrop}
          onClose={goBack}
          size="sm"
          closeOnBackdrop={false}>
          {children}

          <div className={styles.modalButtons}>
            {!isFirstStep && <Button text="Back" color="grey" onClick={() => setStep(step - 1)} />}

            <Button
              text={isLastStep ? 'Got it!' : 'Continue'}
              onClick={() => (isLastStep ? goBack() : setStep(step + 1))}
            />

            <Text size="md">
              {step + 1}/{STEPS.length}
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
            <Segmented className={styles.segmented} options={SEGMENTED_OPTIONS} value="battles" onChange={() => {}} />
          )}
        </Modal>
      </Background>

      <div className={styles.blurBackdrop}>
        {isFirstStep && (
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
