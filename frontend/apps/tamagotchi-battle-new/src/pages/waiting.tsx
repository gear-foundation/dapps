import { Button } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';
import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

import { Card, Loader, Text } from '@/components';
import { Background, WaitList } from '@/features/game/components';
import { Character } from '@/features/game/components/character';
import { CharacterStats } from '@/features/game/components/character-stats';
import { InfoIcon } from '@/features/game/assets/images';
import { mockPlayer1 } from '@/features/game/mock';
import { ROUTES } from '@/app/consts';
import {
  useMyBattleQuery,
  useCancelRegisterMessage,
  useCancelTournamentMessage,
  useStartBattleMessage,
} from '@/app/utils';
import { usePending } from '@/features/game/hooks';
import styles from './waiting.module.scss';

export default function WaitingPage() {
  const navigate = useNavigate();
  const { account } = useAccount();

  const { battleState, isFetching } = useMyBattleQuery();
  console.log('🚀 ~ WaitingPage ~ battleState:', battleState);
  const { cancelTournamentMessage } = useCancelTournamentMessage();
  const { cancelRegisterMessage } = useCancelRegisterMessage();
  const { startBattleMessage } = useStartBattleMessage();

  useEffect(() => {
    if (!isFetching && !battleState) {
      navigate(ROUTES.HOME);
    }
  }, [isFetching, battleState]);

  const { pending } = usePending();

  if (!battleState) {
    return <Loader />;
  }

  const { participants, battle_name, admin } = battleState;
  const me = participants.find(([address]) => address === account.decodedAddress)?.[1];

  if (!me) {
    return <div>Character not found</div>;
  }

  const { appearance, player_settings, user_name } = me;

  const items = participants?.map(([address, { user_name }]) => ({ name: user_name, address }));
  const isAdmin = account.decodedAddress === admin;

  const onStartTournament = () => {
    startBattleMessage({ onSuccess: () => navigate(ROUTES.GAME) });
  };

  const onCancelTournament = () => {
    cancelTournamentMessage({ onSuccess: () => navigate(ROUTES.HOME) });
  };

  const onLeaveGame = () => {
    cancelRegisterMessage({ onSuccess: () => navigate(ROUTES.HOME) });
  };

  // ! TODO replace mockPlayer

  return (
    <>
      <Background>
        <CharacterStats {...mockPlayer1} characterView={appearance} name={user_name} {...player_settings} />
        <div className={styles.character}>
          <Character {...appearance} size="sm" />
        </div>
        <Card
          title={battle_name}
          description="We are waiting for all players to join, create their characters, and for the administrator to start the game."
          size="sm"
          className={styles.card}>
          <WaitList items={items} />
          <div className={styles.footer}>
            <div className={styles.buttons}>
              {isAdmin ? (
                <>
                  <Button
                    text="Cancel tournament"
                    className={styles.redButton}
                    onClick={onCancelTournament}
                    disabled={pending}
                  />
                  <Button text="Start tournament" color="primary" onClick={onStartTournament} disabled={pending} />
                </>
              ) : (
                <Button text="Leave game" className={styles.redButton} onClick={onLeaveGame} disabled={pending} />
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
            <Button
              color="dark"
              text="Show tutorial"
              onClick={() => navigate(ROUTES.ONBOARDING)}
              className={styles.tutorialButton}
              disabled={pending}
            />
          </div>
        </Card>
      </Background>
    </>
  );
}
