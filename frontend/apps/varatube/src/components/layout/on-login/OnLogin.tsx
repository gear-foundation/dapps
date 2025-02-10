import { ReactNode } from 'react';

import { useGetSubscriberQuery } from '@/app/utils/sails/queries';
import { Loader } from '@/components';

type Props = {
  children: ReactNode;
};

function OnLogin({ children }: Props) {
  const { subscriber, isFetched } = useGetSubscriberQuery();

  return !isFetched ? <>{subscriber ? children : <p />}</> : <Loader />;
}

export { OnLogin };
