import { useAccount } from '@gear-js/react-hooks';
import { Loader } from 'components';
import { useSubscriptions } from 'hooks';
import { ReactNode } from 'react';

type Props = {
  children: ReactNode;
};

function OnLogin({ children }: Props) {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const { subscriptionsState, isSubscriptionsStateRead } = useSubscriptions();
  const subscription = subscriptionsState && decodedAddress ? subscriptionsState[decodedAddress] : undefined;

  return isSubscriptionsStateRead ? (
    <>
      {subscription && children} {!subscription && <p />}
    </>
  ) : (
    <Loader />
  );
}

export { OnLogin };
