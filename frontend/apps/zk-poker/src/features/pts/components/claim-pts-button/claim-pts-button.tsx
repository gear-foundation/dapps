import { useMutation } from '@tanstack/react-query';
import clsx from 'clsx';
import { useEffect, useState } from 'react';

import { Button } from '@/components';
import { useGetAccuralMessage, useRemainingTimeQuery } from '@/features/game/sails';

import styles from './claim-pts-button.module.scss';

type Props = {
  onSuccess: () => void;
  ptsBalance?: number;
  className?: string;
};

const ClaimPtsButton = ({ onSuccess, ptsBalance, className }: Props) => {
  const { remainingTime, refetch: refetchRemainingTime, isPending: isPendingRemainingTime } = useRemainingTimeQuery();
  const { getAccuralMessage } = useGetAccuralMessage();
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
    await new Promise((resolve) => setTimeout(resolve, 2000));
    await refetchRemainingTime();
  };

  const { mutate: claimFreePTSMutation, isPending: isPendingClaimFreePTS } = useMutation({
    mutationFn: claimFreePTS,
    onSuccess,
  });

  useEffect(() => {
    if (currentTime === null) {
      setTimeout(() => {
        void refetchRemainingTime();
      }, 1000);
    }
  }, [currentTime, refetchRemainingTime]);

  const formattedTime = currentTime && currentTime > 0 ? `(${new Date(currentTime).toISOString().slice(11, 19)})` : '';
  const isClaimDisabled = !!currentTime || isPendingRemainingTime || remainingTime !== null || isPendingClaimFreePTS;
  const showPulse = !ptsBalance && !isClaimDisabled;

  return (
    <Button
      className={clsx(className, showPulse && styles.pulseButton)}
      onClick={() => claimFreePTSMutation()}
      disabled={isClaimDisabled}>
      Claim your free PTS {formattedTime}
      {showPulse && (
        <>
          <span className={styles.ring} style={{ '--i': 1 } as React.CSSProperties}></span>
          <span className={styles.ring} style={{ '--i': 2 } as React.CSSProperties}></span>
          <span className={styles.ring} style={{ '--i': 3 } as React.CSSProperties}></span>
        </>
      )}
    </Button>
  );
};

export { ClaimPtsButton };
