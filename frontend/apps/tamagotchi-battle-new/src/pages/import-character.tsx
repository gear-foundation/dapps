import clsx from 'clsx';
import { useSetAtom } from 'jotai';
import { Input, Button } from '@gear-js/vara-ui';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { Card, CardButton } from '@/components';
import { Background, CharacterStatsForm, Character } from '@/features/game/components';
import { AdminIcon, SearchIcon } from '@/features/game/assets/images';
import { ROUTES } from '@/app/consts';
import { useGetAppearanceQuery } from '@/app/utils/sails/queries/use-get-appearance-query';
import { CharacterStatsFormValues } from '@/features/game/types';
import { characterAtom, characterStorage } from '@/features/game/store';

import styles from './import-character.module.scss';

export default function ImportCharacter() {
  const navigate = useNavigate();
  const [address, setAddress] = useState<string>('');

  const { appearance } = useGetAppearanceQuery(address);
  const [characterStats, setCharacterStats] = useState<CharacterStatsFormValues>();
  const [isNextDisabled, setIsNextDisabled] = useState(true);

  const isCharacterFound = Boolean(appearance);

  const setCharacter = useSetAtom(characterAtom);

  const onNextClick = (to: string) => {
    if (!characterStats || !appearance) return;
    const { attack, defence, dodge } = characterStats;
    const character = { attack, defence, dodge, warriorId: address as `0x${string}`, appearance };

    setCharacter(character);
    characterStorage.set(character);
    navigate(to);
  };

  return (
    <>
      <Background>
        <Card
          title="Import Character from Program"
          description="Enter the program ID to review your Tamagotchi."
          size="sm"
          className={clsx(styles.card, isCharacterFound && styles.cardFilled)}
          align="left"
          rightSideSlot={
            isCharacterFound && (
              <CharacterStatsForm
                onValuesChange={(stats, isValid) => {
                  setCharacterStats(stats);
                  setIsNextDisabled(!isValid);
                }}
              />
            )
          }>
          <Input
            type="text"
            placeholder="0x…"
            label="Specify program ID of your Tamagotchi character"
            required
            className="w-full"
            onChange={(e) => setAddress(e.target.value)}
          />
          {isCharacterFound && appearance && (
            <div className={styles.character}>
              <Character size="sm" {...appearance} />
            </div>
          )}
        </Card>
        <div className={styles.container}>
          <div className={styles.buttons}>
            <CardButton
              onClick={() => onNextClick(ROUTES.FIND_GAME)}
              icon={<SearchIcon />}
              title="Find a private game"
              description="To find the game, you need to enter the administrator's address."
              disabled={!isCharacterFound || isNextDisabled}
            />
            <CardButton
              onClick={() => onNextClick(ROUTES.CREATE_GAME)}
              icon={<AdminIcon />}
              title="Create a game in administrator mode"
              description="Create a game and specify your participation rules."
              disabled={!isCharacterFound || isNextDisabled}
            />
          </div>
          <Button text="Back" color="grey" onClick={() => navigate(ROUTES.HOME)} />
        </div>
      </Background>
    </>
  );
}
