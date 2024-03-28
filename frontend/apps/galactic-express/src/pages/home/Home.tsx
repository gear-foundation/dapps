import { useEffect, useState } from 'react';
import { useAtom } from 'jotai';
import { REGISTRATION_STATUS } from 'atoms';
import { useAccount } from '@gear-js/react-hooks';
import { Start, Session, useLaunchState } from 'features/session';
import { Welcome } from 'features/welcome/components/welcome';
import { RequestGame } from 'features/welcome/components/enter-contract-address';
import { SessionPassedInfo } from 'features/session/components/session-passed-info';
import { TextModal } from 'features/session/components/game-not-found-modal';

function Home() {
  const [registrationStatus, setRegistrationStatus] = useAtom(REGISTRATION_STATUS);
  const [isPlayerRemovedModalOpen, setIsPlayerRemovedModalOpen] = useState(false);
  const [isGameCancelledModalOpen, setIsGameCancelledModalOpen] = useState(false);
  const { account } = useAccount();
  const state = useLaunchState();
  const { admin, stage, sessionId, altitude, weather, reward, bid, adminName } = state || {};

  const isSessionEnded = Object.keys(stage || {})[0] === 'Results';

  const rankings = stage?.Results?.rankings;
  const turns = stage?.Results?.turns;
  const participants = stage?.Registration || stage?.Results?.participants;

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
              participants={participants || []}
              session={{
                altitude: altitude || '',
                weather: weather || '',
                reward: reward || '',
                sessionId: sessionId || '',
              }}
              bid={bid}
              isUserAdmin={isUserAdmin}
              adminAddress={admin}
              adminName={adminName || ''}
              userAddress={account?.address || ''}
            />
          )}
          {isSessionEnded && (
            <>
              {rankings?.map((item) => item[0]).includes(account?.decodedAddress || '0x') ? (
                <Session
                  session={{
                    altitude: altitude || '',
                    weather: weather || '',
                    reward: reward || '',
                    sessionId: sessionId || '',
                  }}
                  participants={participants || []}
                  turns={turns || []}
                  rankings={rankings || []}
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
