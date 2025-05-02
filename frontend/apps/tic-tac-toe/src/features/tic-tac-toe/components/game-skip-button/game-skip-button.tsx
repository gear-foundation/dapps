import { useAccount, useAlert } from '@gear-js/react-hooks';
import { useState } from 'react';

import { getErrorMessage } from '@dapps-frontend/ui';

import { Button } from '@/components/ui/button';

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
      console.error(error);
      alert.error(getErrorMessage(error));
      setIsLoading(false);
    }
  };

  return (
    <Button onClick={onSkip} isLoading={isLoading} variant="black">
      Skip
    </Button>
  );
}
