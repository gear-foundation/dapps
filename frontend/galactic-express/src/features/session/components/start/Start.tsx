import { useEffect, useState } from 'react';
import clsx from 'clsx';
import { UserMessageSent, encodeAddress } from '@gear-js/api';
import { Button } from '@gear-js/ui';
import { useAtomValue, useSetAtom } from 'jotai';
import { CURRENT_CONTRACT_ADDRESS_ATOM, IS_CONTRACT_ADDRESS_INITIALIZED_ATOM } from 'atoms';
import { Vec, u8 } from '@polkadot/types';
import { useAccount, useApi, withoutCommas } from '@gear-js/react-hooks';
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

type Props = {
  participants: Participant[];
  session: Session;
  isUserAdmin: boolean;
  userAddress: string;
};

type DecodedReply = {
  Err: string;
};

function Start({ participants, session, isUserAdmin, userAddress }: Props) {
  const { api } = useApi();
  const { account } = useAccount();
  const { decodedAddress } = account || {};
  const currentContractAddress = useAtomValue(CURRENT_CONTRACT_ADDRESS_ATOM);
  const setCurrentContractAddress = useSetAtom(CURRENT_CONTRACT_ADDRESS_ATOM);
  const setIsContractAddressInitialized = useSetAtom(IS_CONTRACT_ADDRESS_INITIALIZED_ATOM);
  const { altitude, weather, fuelPrice, reward, sessionId } = session;
  const playersCount = participants?.length ? participants.length + 1 : 1;
  const isRegistered = decodedAddress ? !!participants.some((participant) => participant[0] === decodedAddress) : false;

  const containerClassName = clsx(styles.container, decodedAddress ? styles.smallMargin : styles.largeMargin);

  const [registrationStatus, setRegistrationStatus] = useState<
    'registration' | 'success' | 'error' | 'NotEnoughParticipants' | 'MaximumPlayersReached'
  >('registration');

  const meta = useEscrowMetadata();
  const getDecodedPayload = (payload: Vec<u8>) => {
    if (meta?.types.handle.output) {
      return meta.createType(meta.types.handle.output, payload).toHuman();
    }
  };

  const getDecodedReply = (payload: Vec<u8>): DecodedReply => {
    const decodedPayload = getDecodedPayload(payload);

    return decodedPayload as DecodedReply;
  };

  const handleGoBack = () => {
    setCurrentContractAddress('');
    setIsContractAddressInitialized(false);
  };

  const handleEvents = ({ data }: UserMessageSent) => {
    const { message } = data;
    const { destination, source, payload } = message;
    const isOwner = destination.toHex() === account?.decodedAddress;
    const isEscrowProgram = source.toHex() === currentContractAddress;

    if (isOwner && isEscrowProgram) {
      const reply = getDecodedReply(payload);
      // console.log(reply);
      if (reply?.Err) {
        if (reply.Err === 'NotEnoughParticipants' || reply.Err === 'MaximumPlayersReached') {
          setRegistrationStatus(reply.Err);
        } else {
          setRegistrationStatus('error');
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
  }, [participants, registrationStatus]);

  return (
    <div className={styles.mainContainer}>
      <div>
        <header className={styles.header}>
          <h2 className={styles.heading}>Session #{sessionId}</h2>

          <div>
            <p className={styles.registration}>Registration</p>
            <p className={styles.subheading}>Rockets ({playersCount}/4). Waiting for other players...</p>
          </div>
        </header>

        <Container className={containerClassName}>
          {isUserAdmin && (
            <ParticipantsTable
              data={[
                {
                  id: userAddress,
                  playerAddress: encodeAddress(userAddress),
                },
                ...participants
                  .filter((item) => item[0] !== decodedAddress)
                  .map((item) => ({
                    id: item[0],
                    playerAddress: encodeAddress(item[0]),
                  })),
              ]}
              userAddress={userAddress}
            />
          )}
          <Traits altitude={altitude} weather={weather} fuelPrice={fuelPrice} reward={reward} />

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
              <Form
                weather={weather}
                defaultDeposit={withoutCommas('0')}
                isAdmin={isUserAdmin}
                setRegistrationStatus={setRegistrationStatus}
              />
            )}
          </footer>
        </Container>
      </div>

      <div className={styles.imageWrapper}>
        <img src={src} alt="" className={styles.image} />
      </div>
    </div>
  );
}

export { Start };
