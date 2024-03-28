import { useEffect, useState } from 'react';
import clsx from 'clsx';
import { HexString, UserMessageSent, encodeAddress } from '@gear-js/api';
import { Button } from '@gear-js/ui';
import { useAtom, useSetAtom } from 'jotai';
import { CURRENT_GAME_ATOM, REGISTRATION_STATUS } from 'atoms';
import { ADDRESS } from 'consts';
import { Bytes } from '@polkadot/types';
import { useAccount, useApi } from '@gear-js/react-hooks';
import { UnsubscribePromise } from '@polkadot/api/types';
import src from 'assets/images/earth.gif';
import { Container } from 'components';
import { Participant, Session } from '../../types';
import { Traits } from '../traits';
import { Form } from '../form';
import styles from './Start.module.scss';
import { useEscrowMetadata } from '../../api';
import { ParticipantsTable } from '../participants-table';

import { SuccessfullyRegisteredInfo } from '../successfully-registered-info';
import { Warning } from '../warning';
import { CancelGameButton } from '../cancel-game-button/CancelGameButton';

type Props = {
  participants: Participant[];
  session: Session;
  isUserAdmin: boolean;
  userAddress: string;
  adminAddress: HexString | undefined;
  adminName: string;
  bid: string | undefined;
};

type DecodedReplyOk = {
  playerId: string;
};

type DecodedReply = {
  Err: string;
  Ok: Record<string, DecodedReplyOk> & 'GameCanceled';
};

function Start({ participants, session, isUserAdmin, userAddress, adminAddress, bid, adminName }: Props) {
  const { api } = useApi();
  const { account } = useAccount();
  const { decodedAddress } = account || {};
  const [registrationStatus, setRegistrationStatus] = useAtom(REGISTRATION_STATUS);
  const setCurrentGame = useSetAtom(CURRENT_GAME_ATOM);
  const { altitude, weather, reward, sessionId } = session;
  const playersCount = participants?.length ? participants.length + 1 : 1;
  const isRegistered = decodedAddress ? !!participants.some((participant) => participant[0] === decodedAddress) : false;

  const containerClassName = clsx(styles.container, decodedAddress ? styles.smallMargin : styles.largeMargin);

  const meta = useEscrowMetadata();
  const getDecodedPayload = (payload: Bytes) => {
    if (meta?.types.handle.output) {
      return meta.createType(meta.types.handle.output, payload).toHuman();
    }
  };

  const getDecodedReply = (payload: Bytes): DecodedReply => {
    const decodedPayload = getDecodedPayload(payload);

    return decodedPayload as DecodedReply;
  };

  const handleGoBack = () => {
    setCurrentGame('');
  };

  const handleEvents = ({ data }: UserMessageSent) => {
    const { message } = data;
    const { destination, source, payload } = message;
    const isOwner = destination.toHex() === account?.decodedAddress;
    const isEscrowProgram = source.toHex() === ADDRESS.CONTRACT;

    if (isOwner && isEscrowProgram) {
      const reply = getDecodedReply(payload);

      if (reply?.Err) {
        if (reply.Err === 'NotEnoughParticipants' || reply.Err === 'MaximumPlayersReached') {
          setRegistrationStatus(reply.Err);
          return;
        }

        setRegistrationStatus('error');
      }
    }

    if (destination.toHex() === adminAddress) {
      const reply = getDecodedReply(payload);

      if (reply.Ok) {
        if (reply.Ok.PlayerDeleted?.playerId === account?.decodedAddress) {
          setRegistrationStatus('PlayerRemoved');
        }

        if (reply.Ok === 'GameCanceled' && !isUserAdmin) {
          setRegistrationStatus('GameCanceled');
        }
      }
    }
  };

  useEffect(() => {
    let unsub: UnsubscribePromise | undefined;

    if (api && decodedAddress && meta) {
      unsub = api.gearEvents.subscribeToGearEvent('UserMessageSent', handleEvents);
    }

    return () => {
      if (unsub) unsub.then((unsubCallback) => unsubCallback());
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, decodedAddress, meta]);

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
                playerAddress: encodeAddress(adminAddress || ''),
                playerName: adminName,
              },
              ...participants.map((item) => ({
                id: item[0],
                playerAddress: encodeAddress(item[0]),
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
        <img src={src} alt="earth" className={styles.image} />
      </div>
    </div>
  );
}

export { Start };
