import { useBattle } from 'features/battle/context';
import { useFetchVoucher } from 'app/hooks/use-fetch-voucher';
import { useAccount } from '@gear-js/react-hooks';
import { useCheckBalance } from 'features/wallet/hooks';
import { useBattleMessage } from 'features/battle/hooks';
import { GAS_LIMIT } from 'app/consts';
import { Button } from '@gear-js/ui';

export const NewGameButton = () => {
  const { account } = useAccount();
  const { isPending, setIsPending } = useBattle();
  const { isVoucher, updateBalance } = useFetchVoucher(account?.address);
  const { checkBalance } = useCheckBalance(isVoucher);
  const handleMessage = useBattleMessage();

  const onSuccess = () => setIsPending(false);
  const onError = () => setIsPending(false);

  const handler = async () => {
    const payload = { StartRegistration: null };

    setIsPending(true);

    await updateBalance();

    checkBalance(
      GAS_LIMIT,
      () => {
        handleMessage({ payload, onSuccess, onError, withVoucher: isVoucher });
      },
      onError,
    );
  };

  return <Button text="Start New Game" color="primary" onClick={handler} disabled={isPending} />;
};
