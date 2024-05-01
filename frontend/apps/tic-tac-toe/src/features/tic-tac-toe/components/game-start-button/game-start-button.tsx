import { Button } from '@/components/ui/button';
import { useGameMessage, useHandleCalculateGas, useSubscriptionOnGameMessage } from '../../hooks';
import { useEffect } from 'react';
import { BaseComponentProps } from '@/app/types';
import { useCheckBalance } from '@dapps-frontend/hooks';
import { useAccount, useAlert } from '@gear-js/react-hooks';
import { ADDRESS } from '../../consts';
import { withoutCommas } from '@/app/utils';
import { ProgramMetadata } from '@gear-js/api';
import { useGaslessTransactions, useSignlessTransactions } from '@dapps-frontend/ez-transactions';
import { useAtom } from 'jotai';
import { stateChangeLoadingAtom } from '../../store';

type GameStartButtonProps = BaseComponentProps & {
  meta: ProgramMetadata;
};

export function GameStartButton({ children, meta }: GameStartButtonProps) {
  const calculateGas = useHandleCalculateGas(ADDRESS.GAME, meta);
  const message = useGameMessage(meta);
  const { account } = useAccount();
  const alert = useAlert();

  const signless = useSignlessTransactions();
  const gasless = useGaslessTransactions();

  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });

  const [isLoading, setIsLoading] = useAtom(stateChangeLoadingAtom);
  const { subscribe, unsubscribe, isOpened } = useSubscriptionOnGameMessage(meta);

  useEffect(() => {
    setIsLoading(isOpened);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isOpened]);

  const onError = () => {
    setIsLoading(false);
    unsubscribe();
  };

  const onGameStart = async () => {
    if (!meta || !account || !ADDRESS.GAME) {
      return;
    }
    const payload = { StartGame: {} };
    setIsLoading(true);

    let voucherId = gasless.voucherId;
    if (gasless.isEnabled && !gasless.voucherId && !signless.isActive) {
      voucherId = await gasless.requestVoucher(account.address);
    }

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const minLimit = withoutCommas(min_limit as string);
        const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2);

        subscribe();
        checkBalance(
          gasLimit,
          () => {
            message({
              payload,
              voucherId,
              gasLimit,
              onError,
            });
          },
          onError,
        );
      })
      .catch((error) => {
        onError();
        console.log(error);
        alert.error('Gas calculation error');
      });
  };

  return (
    <Button onClick={onGameStart} isLoading={isLoading || !meta || !ADDRESS.GAME || !account || gasless.isLoading}>
      {children}
    </Button>
  );
}
