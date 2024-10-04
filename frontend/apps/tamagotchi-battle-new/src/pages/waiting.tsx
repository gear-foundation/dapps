import { Button } from '@gear-js/vara-ui';
import { useNavigate } from 'react-router-dom';

import { Card, Text } from '@/components';
import { Background, WaitList } from '@/features/game/components';
import { Character } from '@/features/game/components/character';
import { CharacterStats } from '@/features/game/components/character-stats';
import { InfoIcon } from '@/features/game/assets/images';
import { mockPlayer1 } from '@/features/game/mock';
import { mockCharacterView } from '@/features/game/consts';
import styles from './waiting.module.scss';
import { ROUTES } from '@/app/consts';

export default function WaitingPage() {
  const navigate = useNavigate();

  const tournamentName = 'Tournament name';
  const isAdmin = true;

  const items = [
    { name: 'Real my Name', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
    { name: '123', address: '5CGAN5KR2kMxgsrP3Bx9HFMFYJug23v9V3CmDmZGiXDvZrv8' },
  ];

  const onCancelTournament = () => {
    navigate(-1);
  };
  const onStartTournament = () => {};
  const onLeaveGame = () => {};

  return (
    <>
      <Background>
        <CharacterStats {...mockPlayer1} characterView={mockCharacterView} />
        <div className={styles.character}>
          <Character {...mockCharacterView} />
        </div>
        <Card
          title={tournamentName}
          description="We are waiting for all players to join, create their characters, and for the administrator to start the game."
          size="sm"
          className={styles.card}>
          <WaitList items={items} />
          <div className={styles.footer}>
            <div className={styles.buttons}>
              {isAdmin ? (
                <>
                  <Button text="Cancel tournament" className={styles.redButton} onClick={onCancelTournament} />
                  <Button text="Start tournament" color="primary" onClick={onStartTournament} />
                </>
              ) : (
                <Button text="Leave game" className={styles.redButton} onClick={onLeaveGame} />
              )}
            </div>
            <Text size="xs" weight="medium" className={styles.info}>
              <InfoIcon />
              To change your character, you need to leave the game.
            </Text>
          </div>
        </Card>

        <Card
          title="Something unclear?"
          description="Check out our tutorial. It will help you get started and answer your questions."
          size="sm"
          align="left"
          className={styles.tutorial}>
          <div className={styles.buttons}>
            <Button color="dark" text="Show tutorial" onClick={() => navigate(ROUTES.ONBOARDING)} className={styles.tutorialButton} />
          </div>
        </Card>
      </Background>
    </>
  );
}
