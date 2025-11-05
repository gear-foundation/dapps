import { useApi } from '@gear-js/react-hooks';
import { Button } from '@gear-js/ui';
import { useGaslessTransactions } from 'gear-ez-transactions';

import { useCheckBalance } from '@dapps-frontend/hooks';

import { GAS_LIMIT } from '@/app/consts';
import { gasLimitToNumber } from '@/app/utils';
import { useBattle } from '@/features/battle/context';
import { useBattleMessage } from '@/features/battle/hooks';

export const NewGameButton = () => {
  const { api } = useApi();
  const { isPending, setIsPending } = useBattle();
  const gasless = useGaslessTransactions();
  const { checkBalance } = useCheckBalance({ gaslessVoucherId: gasless.voucherId });
  const handleMessage = useBattleMessage();

  const onSuccess = () => setIsPending(false);
  const onError = () => setIsPending(false);

  const handler = () => {
    const payload = { StartRegistration: null };

    setIsPending(true);

    checkBalance(
      gasLimitToNumber(api?.blockGasLimit),
      () => {
        void handleMessage({
          payload,
          onSuccess,
          onError,
          voucherId: gasless.voucherId,
          gasLimit: GAS_LIMIT,
        });
      },
      onError,
    );
  };

  return <Button text="Start New Game" color="primary" onClick={handler} disabled={isPending || gasless.isLoading} />;
};
