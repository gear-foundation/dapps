import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';
import { useSetAtom } from 'jotai';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { Card, CardButton } from '@/components';
import { AdminIcon, SearchIcon } from '@/features/game/assets/images';
import { Background, CharacterStatsForm, Character } from '@/features/game/components';
import { CharacterView } from '@/features/game/components/character/character';
import {
  characterAppearanceAtom,
  characterAppearanceStorage,
  characterStatsStorage,
  warriorIdStorage,
} from '@/features/game/store';
import { CharacterStatsFormValues } from '@/features/game/types';
import { generateRandomCharacterView } from '@/features/game/utils';

import styles from './generate-character.module.scss';

export default function GenerateCharacter() {
  const navigate = useNavigate();
  const [characterView, setCharacterView] = useState<CharacterView>(
    characterAppearanceStorage.get() || generateRandomCharacterView(),
  );
  const [prevCharacterView, setPrevCharacterView] = useState<CharacterView | null>(generateRandomCharacterView());
  const [characterStats, setCharacterStats] = useState<CharacterStatsFormValues | null>(characterStatsStorage.get());
  const [isNextDisabled, setIsNextDisabled] = useState(true);

  const generate = () => {
    setPrevCharacterView(characterView);
    setCharacterView(generateRandomCharacterView());
  };

  const setCharacterAppearance = useSetAtom(characterAppearanceAtom);

  const onNextClick = (to: string) => {
    if (!characterStats) return;
    setCharacterAppearance(characterView);
    characterStatsStorage.set(characterStats);
    characterAppearanceStorage.set(characterView);
    warriorIdStorage.set(null);
    navigate(to);
  };

  return (
    <>
      <Background>
        <Card
          title="Generate Character Without a Code"
          description="Click ‘Generate’ to choose your character’s appearance."
          size="sm"
          className={clsx(styles.card, styles.cardFilled)}
          align="left"
          rightSideSlot={
            <CharacterStatsForm
              onValuesChange={(stats, isValid) => {
                setCharacterStats(stats);
                setIsNextDisabled(!isValid);
              }}
            />
          }>
          <div className={styles.character}>
            <Character
              {...characterView}
              loaderBackground={true}
              size="sm"
              fallback={prevCharacterView && <Character {...prevCharacterView} withSpiner={false} size="sm" />}
            />
          </div>
          <Button text="Generate" color="dark" onClick={generate} className={styles.generate} />
        </Card>
        <div className={styles.container}>
          <div className={styles.buttons}>
            <CardButton
              onClick={() => onNextClick(ROUTES.FIND_GAME)}
              icon={<SearchIcon />}
              title="Find a private game"
              description="To find the game, you need to enter the administrator's address."
              disabled={isNextDisabled}
            />
            <CardButton
              onClick={() => onNextClick(ROUTES.CREATE_GAME)}
              icon={<AdminIcon />}
              title="Create a game in administrator mode"
              description="Create a game and specify your participation rules."
              disabled={isNextDisabled}
            />
          </div>
          <Button text="Back" color="grey" onClick={() => navigate(-1)} />
        </div>
      </Background>
    </>
  );
}
