import { HexString } from '@gear-js/api';
import { getVaraAddress, useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/ui';
import clsx from 'clsx';
import { useAtom, useSetAtom } from 'jotai';
import { useEffect } from 'react';

import { useEventGameCanceledSubscription, useEventPlayerDeletedSubscription } from '@/app/utils';
import earthGif from '@/assets/images/earth.gif';
import { CURRENT_GAME_ATOM, REGISTRATION_STATUS } from '@/atoms';
import { Container } from '@/components';

import { Participant, Session } from '../../types';
import { CancelGameButton } from '../cancel-game-button/CancelGameButton';
import { Form } from '../form';
import { ParticipantsTable } from '../participants-table';
import { SuccessfullyRegisteredInfo } from '../successfully-registered-info';
import { Traits } from '../traits';
import { Warning } from '../warning';

import styles from './Start.module.scss';

type Props = {
  participants: Participant[];
  session: Session;
  isUserAdmin: boolean;
  userAddress: string;
  adminAddress: HexString | undefined;
  adminName: string;
  bid: string | undefined;
};

function Start({ participants, session, isUserAdmin, userAddress, adminAddress, bid, adminName }: Props) {
  const { account } = useAccount();
  const { decodedAddress } = account || {};
  const [registrationStatus, setRegistrationStatus] = useAtom(REGISTRATION_STATUS);
  const setCurrentGame = useSetAtom(CURRENT_GAME_ATOM);
  const { altitude, weather, reward } = session;
  const playersCount = participants?.length ? participants.length + 1 : 1;
  const isRegistered = decodedAddress ? !!participants.some((participant) => participant[0] === decodedAddress) : false;
  const containerClassName = clsx(styles.container, decodedAddress ? styles.smallMargin : styles.largeMargin);

  useEventGameCanceledSubscription(isUserAdmin);
  useEventPlayerDeletedSubscription();

  const handleGoBack = () => {
    setCurrentGame(null);
  };

  useEffect(() => {
    if (registrationStatus === 'NotEnoughParticipants' && participants.length) {
      setRegistrationStatus('registration');
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [participants, registrationStatus]);

  return (
    <div className={styles.mainContainer}>
      <div>
        <header className={styles.header}>
          <h2 className={styles.heading}>Session</h2>

          <div>
            <p className={styles.registration}>Registration</p>
            <p className={styles.subheading}>Rockets ({playersCount}/4). Waiting for other players...</p>
          </div>
        </header>

        <Container className={containerClassName}>
          <ParticipantsTable
            data={[
              {
                id: adminAddress || '',
                playerAddress: getVaraAddress(adminAddress || ''),
                playerName: adminName,
              },
              ...participants.map((item) => ({
                id: item[0],
                playerAddress: getVaraAddress(item[0]),
                playerName: item[1].name,
              })),
            ]}
            userAddress={userAddress}
            isUserAdmin={isUserAdmin}
          />

          <Traits altitude={altitude} weather={weather} reward={reward} />

          <footer>
            {isRegistered && !isUserAdmin && <SuccessfullyRegisteredInfo />}
            {!participants.length &&
              isUserAdmin &&
              registrationStatus === 'NotEnoughParticipants' &&
              participants.length < 2 && (
                <Warning title="Not Enough Participants" text="At least two players should participate." />
              )}
            {!isUserAdmin && registrationStatus === 'MaximumPlayersReached' && (
              <div className={styles.errorWrapper}>
                <Warning
                  title="Maximum number of players reached"
                  text="Please try again later or choose another contract address."
                />
                <Button text="Back" onClick={handleGoBack} />
              </div>
            )}
            {!isRegistered && registrationStatus === 'error' && !isUserAdmin && (
              <Warning title="Error" text="Please try again later or choose another contract address." />
            )}
            {((isUserAdmin && registrationStatus !== 'NotEnoughParticipants') ||
              (!isUserAdmin && !isRegistered && registrationStatus === 'registration')) && (
              <Form weather={weather} bid={bid} isAdmin={isUserAdmin} setRegistrationStatus={setRegistrationStatus} />
            )}
          </footer>
        </Container>
      </div>

      <div className={styles.imageWrapper}>
        {isRegistered && !isUserAdmin && <CancelGameButton isAdmin={isUserAdmin} participants={participants} />}
        <img src={earthGif} alt="earth" className={styles.image} />
      </div>
    </div>
  );
}

export { Start };
