import { useAccount } from '@gear-js/react-hooks';
import { useAtom } from 'jotai';
import { useEffect, useState } from 'react';

import { REGISTRATION_STATUS } from '@/atoms';
import { Start, Session, useLaunchState } from '@/features/session';
import { TextModal } from '@/features/session/components/game-not-found-modal';
import { SessionPassedInfo } from '@/features/session/components/session-passed-info';
import { RequestGame } from '@/features/welcome/components/enter-contract-address';
import { Welcome } from '@/features/welcome/components/welcome';

function Home() {
  const [registrationStatus, setRegistrationStatus] = useAtom(REGISTRATION_STATUS);
  const [isPlayerRemovedModalOpen, setIsPlayerRemovedModalOpen] = useState(false);
  const [isGameCancelledModalOpen, setIsGameCancelledModalOpen] = useState(false);
  const { account } = useAccount();
  const state = useLaunchState();
  const { admin, stage, altitude, weather, reward, bid, admin_name } = state || {};

  const isSessionEnded = stage && 'results' in stage;

  const rankings = isSessionEnded ? stage.results.rankings : [];
  const turns = isSessionEnded ? stage.results.turns : [];

  const registrationParticipants = stage && 'registration' in stage && stage.registration;
  const resultsParticipants = isSessionEnded && stage.results.participants;
  const participants = registrationParticipants || resultsParticipants || [];

  const isUserAdmin = admin === account?.decodedAddress;

  useEffect(() => {
    if (registrationStatus === 'PlayerRemoved') {
      setIsPlayerRemovedModalOpen(true);
    }

    if (registrationStatus === 'GameCanceled') {
      setIsGameCancelledModalOpen(true);
    }
  }, [registrationStatus]);

  const handleCloseModal = () => {
    setIsPlayerRemovedModalOpen(false);
    setIsGameCancelledModalOpen(false);
    setRegistrationStatus('registration');
  };

  return (
    <>
      {!state && (
        <Welcome>
          <RequestGame />
        </Welcome>
      )}
      {!!state && (
        <>
          {!isSessionEnded && (
            <Start
              participants={participants}
              session={{
                altitude: String(altitude || ''),
                weather: weather || '',
                reward: String(reward || ''),
              }}
              bid={String(bid || 0)}
              isUserAdmin={isUserAdmin}
              adminAddress={admin}
              adminName={admin_name || ''}
              userAddress={account?.address || ''}
            />
          )}
          {isSessionEnded && (
            <>
              {rankings?.map((item) => item[0]).includes(account?.decodedAddress || '0x') ? (
                <Session
                  session={{
                    altitude: String(altitude || ''),
                    weather: weather || '',
                    reward: String(reward || ''),
                  }}
                  participants={participants || []}
                  turns={turns}
                  rankings={rankings}
                  userId={account?.decodedAddress}
                  admin={admin}
                />
              ) : (
                <SessionPassedInfo />
              )}
            </>
          )}
        </>
      )}
      {isPlayerRemovedModalOpen && (
        <TextModal
          heading="You have left the game"
          text="The administrator has removed you from the player list"
          onClose={handleCloseModal}
        />
      )}
      {isGameCancelledModalOpen && (
        <TextModal
          heading="The game has been canceled by the administrator"
          text="Game administrator Samovit has ended the game. All spent VARA tokens for the entry fee will be refunded."
          onClose={handleCloseModal}
        />
      )}
    </>
  );
}

export { Home };
