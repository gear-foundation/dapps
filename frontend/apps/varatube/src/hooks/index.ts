import { useAccount } from '@gear-js/react-hooks';
import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useProgramState, useSubscriptionsMessage } from './api';

function useSubscription() {
  const navigate = useNavigate();

  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const { subscriptionsState, isSubscriptionsStateRead } = useProgramState();

  const subscription = subscriptionsState && decodedAddress ? subscriptionsState[decodedAddress] : undefined;

  useEffect(() => {
    if (isSubscriptionsStateRead && !subscription) {
      navigate('/subscription');
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isSubscriptionsStateRead, subscription, account]);

  return isSubscriptionsStateRead;
}

export { useSubscriptionsMessage, useSubscription };
