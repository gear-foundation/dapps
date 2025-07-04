import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import {
  useMyBattleQuery,
  useCancelRegisterMessage,
  useCancelTournamentMessage,
  useStartBattleMessage,
} from '@/app/utils';
import { useEventBattleCanceledSubscription } from '@/app/utils/sails/events';
import { Card, Loader, Text, Modal } from '@/components';
import { InfoIcon } from '@/features/game/assets/images';
import { Background, GameCanceledModal, WaitList } from '@/features/game/components';
import { Character } from '@/features/game/components/character';
import { CharacterStats } from '@/features/game/components/character-stats';
import { usePending, useResetGameState } from '@/features/game/hooks';

import styles from './waiting.module.scss';

export function Waiting() {
  const navigate = useNavigate();
  const { account } = useAccount();

  const { battleState, isFetching } = useMyBattleQuery();
  const { cancelTournamentMessage } = useCancelTournamentMessage();
  const { cancelRegisterMessage } = useCancelRegisterMessage();
  const { startBattleMessage } = useStartBattleMessage();
  const { isBattleCanceled } = useEventBattleCanceledSubscription(battleState?.admin);

  const [isOpenLeaveModal, setIsOpenLeaveModal] = useState(false);
  const [isOpenCancelTournamentModal, setIsOpenCancelTournamentModal] = useState(false);

  useResetGameState();

  useEffect(() => {
    if (!isFetching && !battleState && !isBattleCanceled) {
      navigate(ROUTES.HOME);
    }
  }, [isFetching, battleState, isBattleCanceled, navigate]);

  const { pending } = usePending();

  if (isBattleCanceled) {
    return <GameCanceledModal />;
  }

  if (!battleState || !account) {
    return <Loader />;
  }

  const { participants, battle_name, admin } = battleState;
  const player = participants.find(([address]) => address === account.decodedAddress)?.[1];

  if (!player) {
    return <div>Character not found</div>;
  }

  const { appearance, player_settings, user_name } = player;

  const items = participants?.map(([address, participant]) => ({ name: participant.user_name, address }));
  const isAdmin = account.decodedAddress === admin;

  const onStartTournament = () => {
    startBattleMessage({ onSuccess: () => navigate(ROUTES.GAME) });
  };

  const onCancelTournament = () => {
    cancelTournamentMessage({
      onSuccess: () => navigate(ROUTES.HOME),
      onError: () => setIsOpenCancelTournamentModal(false),
    });
  };

  const onLeaveGame = () => {
    cancelRegisterMessage({ onSuccess: () => navigate(ROUTES.HOME), onError: () => setIsOpenLeaveModal(false) });
  };

  return (
    <>
      <Background className={styles.background}>
        <CharacterStats characterView={appearance} name={user_name} {...player_settings} />
        <div className={styles.character}>
          <Character {...appearance} size="sm" />
        </div>
        <Card
          title={battle_name}
          description="We are waiting for all players to join, create their characters, and for the administrator to start the game."
          size="sm"
          className={styles.card}>
          <WaitList items={items} isAdmin={isAdmin} />
          <div className={styles.footer}>
            <div className={styles.buttons}>
              {isAdmin ? (
                <>
                  <Button
                    text="Cancel tournament"
                    className={styles.redButton}
                    onClick={() => setIsOpenCancelTournamentModal(true)}
                    disabled={pending}
                  />
                  <Button
                    text="Start tournament"
                    color="primary"
                    onClick={onStartTournament}
                    disabled={pending || items.length < 2}
                  />
                </>
              ) : (
                <Button
                  text="Leave game"
                  className={styles.redButton}
                  onClick={() => setIsOpenLeaveModal(true)}
                  disabled={pending}
                />
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
              color="contrast"
              text="Show tutorial"
              onClick={() => navigate(ROUTES.ONBOARDING)}
              className={styles.tutorialButton}
              disabled={pending}
            />
          </div>
        </Card>

        {isOpenLeaveModal && (
          <Modal
            title="Confirm action"
            description="Are you sure you want to leave the game? You’ll need to reconnect and select your character again."
            className={styles.leaveModal}
            onClose={() => setIsOpenLeaveModal(false)}
            buttons={
              <>
                <Button color="grey" text="Back" onClick={() => setIsOpenLeaveModal(false)} />
                <Button className={styles.redButton} text="Leave game" onClick={onLeaveGame} />
              </>
            }
          />
        )}

        {isOpenCancelTournamentModal && (
          <Modal
            title="Sure you want to end the game?"
            description="This action cannot be undone. The game will be concluded, and all players will exit the gaming room. Any entry fees will be refunded to all players."
            className={styles.cancelTournamentModal}
            onClose={() => setIsOpenCancelTournamentModal(false)}
            buttons={
              <>
                <Button color="grey" text="End tournament" onClick={onCancelTournament} />
                <Button
                  color="primary"
                  text="Continue tournament"
                  onClick={() => setIsOpenCancelTournamentModal(false)}
                />
              </>
            }
          />
        )}
      </Background>
    </>
  );
}
