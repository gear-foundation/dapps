import { Button } from '@/components/ui/button';
import { useGameMessage, useSubscriptionOnGameMessage } from '../../hooks';
import { useEffect, useState } from 'react';
import { useAccount, useAlert, useHandleCalculateGas } from '@gear-js/react-hooks';
import { ADDRESS } from '../../consts';
import { withoutCommas } from '@/app/utils';
import { ProgramMetadata } from '@gear-js/api';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { useCheckBalance } from '@dapps-frontend/hooks';

type Props = {
  meta: ProgramMetadata;
};

export function GameSkipButton({ meta }: Props) {
  const calculateGas = useHandleCalculateGas(ADDRESS.GAME, meta);
  const message = useGameMessage(meta);
  const alert = useAlert();
  const { account } = useAccount();

  const { signless, gasless } = useEzTransactions();

  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });

  const [isLoading, setIsLoading] = useState<boolean>(false);
  const { subscribe, unsubscribe, isOpened } = useSubscriptionOnGameMessage(meta);

  useEffect(() => {
    setIsLoading(isOpened);
  }, [isOpened]);

  const onError = () => {
    setIsLoading(false);
    unsubscribe();
  };

  const onSuccess = () => {
    setIsLoading(false);
  };

  const onNextTurn = async () => {
    if (!meta || !account || !ADDRESS.GAME) {
      return;
    }

    const payload = { Skip: {} };
    setIsLoading(true);

    let voucherId = gasless.voucherId;
    if (account && gasless.isEnabled && !gasless.voucherId && !signless.isActive) {
      voucherId = await gasless.requestVoucher(account.address);
    }

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string);
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2);

        subscribe();

        const sendMessage = () => message({ payload, gasLimit, voucherId, onError, onSuccess });
        if (voucherId) {
          sendMessage();
        } else {
          checkBalance(gasLimit, sendMessage, onError);
        }
      })
      .catch((error) => {
        onError();
        console.log(error);
        alert.error('Gas calculation error');
      });
  };

  return (
    <Button onClick={onNextTurn} isLoading={isLoading} variant="black">
      Skip
    </Button>
  );
}
