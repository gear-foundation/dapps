import { useAccount } from '@gear-js/react-hooks';
import { Loader } from 'components';
import { useProgramState } from 'hooks/api';

import { ReactNode } from 'react';

type Props = {
  children: ReactNode;
};

function OnLogin({ children }: Props) {
  const { account } = useAccount();
  const { decodedAddress } = account || {};

  const { subscriptionsState, isSubscriptionsStateRead } = useProgramState();
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
