import { Input, Button } from '@gear-js/vara-ui';
import { useSetAtom } from 'jotai';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Card } from '@/components';
import { CardButton } from '@/components/ui/card-button';
import { gameStatusAtom } from '../../store';
import { Character } from '../character';
import styles from './import-character.module.scss';
import { CharacterStats } from '../character-stats';
import { AdminIcon, SearchIcon } from '../../assets/images';
import { Background } from '../background';

export const ImportCharacter = () => {
  const navigate = useNavigate();
  const setGameStatus = useSetAtom(gameStatusAtom);
  const [address, setAddress] = useState<string>();

  return (
    <>
      <Background>
        <Card title="Import Character from Program" size="sm" className={styles.card}>
          <Input
            type="text"
            placeholder="0x…"
            label="Specify program ID of your Tamagotchi character"
            required
            className="w-full"
            onChange={(e) => setAddress(e.target.value)}
          />
          <div className={styles.character}>
            <Character />
            <CharacterStats />
          </div>
        </Card>
        <div className={styles.container}>
          <div className={styles.buttons}>
            <CardButton
              onClick={() => setGameStatus('find')}
              icon={<AdminIcon />}
              title="Find a private game"
              subTitle="To find the game, you need to enter the administrator's address."
            />
            <CardButton
              onClick={() => setGameStatus('create')}
              icon={<SearchIcon />}
              title="Create a game in administrator mode"
              subTitle="Create a game and specify your participation rules."
            />
          </div>
          <Button text="Back" color="grey" onClick={() => setGameStatus(null)} />
        </div>
      </Background>
    </>
  );
};
