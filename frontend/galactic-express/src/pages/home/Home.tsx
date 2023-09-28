import { Start, Session, useLaunchState } from 'features/session';
import { CURRENT_CONTRACT_ADDRESS_ATOM, IS_CONTRACT_ADDRESS_INITIALIZED_ATOM } from 'atoms';
import { useAtomValue } from 'jotai';
import { Welcome } from 'features/welcome/components/welcome';
import { useAccount } from '@gear-js/react-hooks';
import { EnterContractAddress } from 'features/welcome/components/enter-contract-address';
import { WalletInfo } from 'features/wallet/components';

function Home() {
  const { account } = useAccount();
  const currentContractAddress = useAtomValue(CURRENT_CONTRACT_ADDRESS_ATOM);
  const isContractAddressInitialized = useAtomValue(IS_CONTRACT_ADDRESS_INITIALIZED_ATOM);
  const state = useLaunchState(currentContractAddress);
  const { admin, session, turns, participants, isSessionEnded, rankings } = state || {};

  const { sessionId, altitude, weather, fuelPrice, reward } = session || {};

  const isUserAdmin = admin === account?.decodedAddress;
  const isStateComing = !!state;
  console.log(state);
  return (
    <>
      {(!isContractAddressInitialized || (!Number(sessionId) && isSessionEnded)) && (
        <Welcome>
          {account ? (
            <EnterContractAddress
              doesSessionExist={!isSessionEnded}
              isUserAdmin={isUserAdmin}
              isStateComing={isStateComing}
            />
          ) : (
            <WalletInfo account={account} />
          )}
        </Welcome>
      )}
      {currentContractAddress && isContractAddressInitialized && (
        <>
          {participants ? (
            <>
              {!isSessionEnded && (
                <Start
                  participants={participants}
                  session={{
                    altitude: altitude || '',
                    weather: weather || '',
                    fuelPrice: fuelPrice || '',
                    reward: reward || '',
                    sessionId: sessionId || '',
                  }}
                  isUserAdmin={isUserAdmin}
                  userAddress={account?.address || ''}
                />
              )}
              {isSessionEnded && (
                <Session
                  session={{
                    altitude: altitude || '',
                    weather: weather || '',
                    fuelPrice: fuelPrice || '',
                    reward: reward || '',
                    sessionId: sessionId || '',
                  }}
                  participants={participants}
                  turns={turns || []}
                  rankings={rankings || []}
                  userId={account?.decodedAddress}
                />
              )}
            </>
          ) : (
            <p>Waiting for session to start...</p>
          )}
        </>
      )}
    </>
  );
}

export { Home };
