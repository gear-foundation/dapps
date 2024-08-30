import { Button } from '@/components/ui/button';
import { useState } from 'react';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useEventMoveMadeSubscription, useEventGameFinishedSubscription, useSkipMessage } from '../../sails';

export function GameSkipButton() {
  const { skipMessage } = useSkipMessage();
  const alert = useAlert();
  const { account } = useAccount();
  const [isLoading, setIsLoading] = useState<boolean>(false);

  useEventMoveMadeSubscription();
  useEventGameFinishedSubscription();

  const onSkip = async () => {
    if (!account) {
      return;
    }

    setIsLoading(true);
    try {
      await skipMessage();
    } catch (error) {
      console.log(error);
      alert.error((error instanceof Error && error.message) || 'Game skip error');
      setIsLoading(false);
    }
  };

  return (
    <Button onClick={onSkip} isLoading={isLoading} variant="black">
      Skip
    </Button>
  );
}
