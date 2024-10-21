import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useGetSubscriberQuery } from 'app/utils';

function useSubscription() {
  const navigate = useNavigate();

  const { subscriber, isFetched } = useGetSubscriberQuery();

  useEffect(() => {
    if (isFetched && !subscriber) {
      navigate('/subscription');
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isFetched, subscriber]);

  return Boolean(subscriber);
}

export { useSubscription };
