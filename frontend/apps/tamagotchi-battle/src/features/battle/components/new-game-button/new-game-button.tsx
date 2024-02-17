import { useBattle } from 'features/battle/context';
import { useCheckBalance } from '@dapps-frontend/hooks';
import { useBattleMessage } from 'features/battle/hooks';
import { Button } from '@gear-js/ui';
import { useApi } from '@gear-js/react-hooks';
import { gasLimitToNumber } from 'app/utils';
import { useGaslessTransactions } from '@dapps-frontend/gasless-transactions';
import { GAS_LIMIT } from 'app/consts';

export const NewGameButton = () => {
  const { api } = useApi();
  const { isPending, setIsPending } = useBattle();
  const { voucherId, isLoadingVoucher } = useGaslessTransactions();
  const { checkBalance } = useCheckBalance({ gaslessVoucherId: voucherId });
  const handleMessage = useBattleMessage();

  const onSuccess = () => setIsPending(false);
  const onError = () => setIsPending(false);

  const handler = async () => {
    const payload = { StartRegistration: null };

    setIsPending(true);

    checkBalance(
      gasLimitToNumber(api?.blockGasLimit),
      () => {
        handleMessage({
          payload,
          onSuccess,
          onError,
          voucherId,
          gasLimit: GAS_LIMIT,
        });
      },
      onError,
    );
  };

  return <Button text="Start New Game" color="primary" onClick={handler} disabled={isPending || isLoadingVoucher} />;
};
