import { useEffect, useState } from 'react';

import { Button } from '@/components';
import { useGetAccuralMessage, useRemainingTimeQuery } from '@/features/game/sails';

type Props = {
  onSuccess: () => void;
  className?: string;
};

const ClaimPtsButton = ({ onSuccess, className }: Props) => {
  const { remainingTime, refetch: refetchRemainingTime, isPending: isPendingRemainingTime } = useRemainingTimeQuery();
  const { getAccuralMessage, isPending } = useGetAccuralMessage();
  const [currentTime, setCurrentTime] = useState<number | null>(null);

  useEffect(() => {
    if (remainingTime) {
      setCurrentTime(Number(remainingTime));

      const timer = setInterval(() => {
        setCurrentTime((prev) => (prev && prev > 0 ? prev - 1000 : null));
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

  const formattedTime = currentTime && currentTime > 0 ? `(${new Date(currentTime).toISOString().slice(11, 19)})` : '';
  const isClaimDisabled = !!currentTime || isPendingRemainingTime || isPending;

  return (
    <Button className={className} onClick={claimFreePTS} disabled={isClaimDisabled}>
      Claim your free PTS {formattedTime}
    </Button>
  );
};

export { ClaimPtsButton };
