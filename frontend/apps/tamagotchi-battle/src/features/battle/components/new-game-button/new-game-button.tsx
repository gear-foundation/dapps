import { useBattle } from 'features/battle/context';
import { useFetchVoucher } from 'features/battle/utils/init-gasless-transactions';
import { useCheckBalance } from 'features/wallet/hooks';
import { useBattleMessage } from 'features/battle/hooks';
import { GAS_LIMIT } from 'app/consts';
import { Button } from '@gear-js/ui';

export const NewGameButton = () => {
  const { isPending, setIsPending } = useBattle();
  const { isVoucher, isLoading } = useFetchVoucher();
  const { checkBalance } = useCheckBalance(isVoucher);
  const handleMessage = useBattleMessage();

  const onSuccess = () => setIsPending(false);
  const onError = () => setIsPending(false);

  const handler = async () => {
    const payload = { StartRegistration: null };

    setIsPending(true);

    checkBalance(
      GAS_LIMIT,
      () => {
        handleMessage({ payload, onSuccess, onError, withVoucher: isVoucher });
      },
      onError,
    );
  };

  return <Button text="Start New Game" color="primary" onClick={handler} disabled={isPending || isLoading} />;
};
