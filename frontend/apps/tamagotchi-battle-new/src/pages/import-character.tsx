import { Input, Button } from '@gear-js/vara-ui';
import { useSetAtom } from 'jotai';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { Card, CardButton } from '@/components';
import { Background, CharacterStatsForm, Character } from '@/features/game/components';
import { gameStatusAtom } from '@/features/game/store';
import { AdminIcon, SearchIcon } from '@/features/game/assets/images';
import { ROUTES } from '@/app/consts';

import styles from './import-character.module.scss';
import clsx from 'clsx';
import { mockCharacterView } from '@/features/game/consts';

export default function ImportCharacter() {
  const navigate = useNavigate();
  const setGameStatus = useSetAtom(gameStatusAtom);
  const [address, setAddress] = useState<string>();

  const isCharacterFound = Boolean(true);

  return (
    <>
      <Background>
        <Card
          title="Import Character from Program"
          description="Enter the program ID to review your Tamagotchi."
          size="sm"
          className={clsx(styles.card, isCharacterFound && styles.cardFilled)}
          align="left"
          rightSideSlot={isCharacterFound && <CharacterStatsForm />}>
          <Input
            type="text"
            placeholder="0x…"
            label="Specify program ID of your Tamagotchi character"
            required
            className="w-full"
            onChange={(e) => setAddress(e.target.value)}
          />
          {isCharacterFound && (
            <div className={styles.character}>
              <Character {...mockCharacterView} size='sm' />
            </div>
          )}
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
