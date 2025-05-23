import { useEffect, useState } from 'react';

import { Button } from '@/components';
import { useGetAccuralMessage, useRemainingTimeQuery } from '@/features/game/sails';

type Props = {
  onSuccess: () => void;
  className?: string;
};

const ClaimPtsButton = ({ onSuccess, className }: Props) => {
  const { remainingTime, refetch: refetchRemainingTime, isFetching: isFetchingRemainingTime } = useRemainingTimeQuery();
  const { getAccuralMessage, isPending } = useGetAccuralMessage();
  const [currentTime, setCurrentTime] = useState<number | null>(null);

  useEffect(() => {
    if (remainingTime) {
      const secondInDay = 86400000;
      setCurrentTime(secondInDay - Number(remainingTime));

      const timer = setInterval(() => {
        setCurrentTime((prev) => (prev ? prev - 1000 : null));
      }, 1000);

      return () => clearInterval(timer);
    } else {
      setCurrentTime(null);
    }
  }, [remainingTime]);

  const claimFreePTS = async () => {
    await getAccuralMessage();
    void refetchRemainingTime();
    onSuccess();
  };

  const formattedTime = currentTime ? `(${new Date(currentTime).toISOString().slice(11, 19)})` : '';
  const isClaimDisabled = !!remainingTime || isFetchingRemainingTime || isPending;

  return (
    <Button className={className} onClick={claimFreePTS} disabled={isClaimDisabled}>
      Claim your free PTS {formattedTime}
    </Button>
  );
};

export { ClaimPtsButton };
