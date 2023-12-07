import { useBattle } from 'features/battle/context';
import { useFetchVoucher } from '@dapps-frontend/gasless-transactions';
import { useAccount } from '@gear-js/react-hooks';
import { useCheckBalance } from 'features/wallet/hooks';
import { useBattleMessage } from 'features/battle/hooks';
import { ENV, GAS_LIMIT } from 'app/consts';
import { Button } from '@gear-js/ui';
import { BATTLE_ADDRESS } from 'features/battle/consts';

export const NewGameButton = () => {
  const { account } = useAccount();
  const { isPending, setIsPending } = useBattle();
  const { isVoucher, isLoading } = useFetchVoucher({
    accountAddress: account?.address,
    programId: BATTLE_ADDRESS,
    backendAddress: ENV.BACK,
  });
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
