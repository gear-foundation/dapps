import clsx from 'clsx';
import { Button } from '@gear-js/vara-ui';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { Card, CardButton } from '@/components';
import { Background, CharacterStatsForm, Character } from '@/features/game/components';
import { AdminIcon, SearchIcon } from '@/features/game/assets/images';
import { CharacterView } from '@/features/game/components/character/character';
import { generateRandomCharacterView } from '@/features/game/utils';
import { ROUTES } from '@/app/consts';

import styles from './generate-character.module.scss';

export default function GenerateCharacter() {
  const navigate = useNavigate();
  const [characterView, setCharacterView] = useState<CharacterView>(generateRandomCharacterView());
  const [prevCharacterView, setPrevCharacterView] = useState<CharacterView | null>(generateRandomCharacterView());

  const isCharacterReady = Boolean(true);

  const generate = () => {
    setPrevCharacterView(characterView);
    setCharacterView(generateRandomCharacterView());
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
          rightSideSlot={<CharacterStatsForm />}>
          {isCharacterReady && (
            <div className={styles.character}>
              <Character
                {...characterView}
                fallback={prevCharacterView && <Character {...prevCharacterView} withSpiner={false} />}
              />
            </div>
          )}
          <Button text="Generate" color="dark" onClick={generate} className={styles.generate} />
        </Card>
        <div className={styles.container}>
          <div className={styles.buttons}>
            <CardButton
              onClick={() => navigate(ROUTES.FIND_GAME)}
              icon={<SearchIcon />}
              title="Find a private game"
              description="To find the game, you need to enter the administrator's address."
            />
            <CardButton
              onClick={() => navigate(ROUTES.CREATE_GAME)}
              icon={<AdminIcon />}
              title="Create a game in administrator mode"
              description="Create a game and specify your participation rules."
            />
          </div>
          <Button text="Back" color="grey" onClick={() => navigate(-1)} />
        </div>
      </Background>
    </>
  );
}
