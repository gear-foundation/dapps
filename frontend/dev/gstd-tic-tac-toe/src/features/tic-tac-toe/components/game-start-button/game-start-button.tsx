import { Button } from '@/components/ui/button';
import { useGameMessage, useSubscriptionOnGameMessage } from '../../hooks';
import { useEffect } from 'react';
import { BaseComponentProps } from '@/app/types';
import { useCheckBalance, useDnsProgramIds } from '@dapps-frontend/hooks';
import { useAccount, useAlert, useHandleCalculateGas } from '@gear-js/react-hooks';
import { withoutCommas } from '@/app/utils';
import { ProgramMetadata } from '@gear-js/api';
import { useGaslessTransactions, useSignlessTransactions } from 'gear-ez-transactions';
import { useAtom } from 'jotai';
import { stateChangeLoadingAtom } from '../../store';

type GameStartButtonProps = BaseComponentProps & {
  meta: ProgramMetadata;
};

export function GameStartButton({ children, meta }: GameStartButtonProps) {
  const { programId } = useDnsProgramIds();
  const message = useGameMessage(meta);
  const { account } = useAccount();
  const alert = useAlert();

  const signless = useSignlessTransactions();
  const gasless = useGaslessTransactions();
  const calculateGas = useHandleCalculateGas(programId, meta);

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
    if (!meta || !account || !programId) {
      return;
    }
    const payload = { StartGame: {} };
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

        const sendMessage = () => message({ payload, gasLimit, voucherId, onError });
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
    <Button onClick={onGameStart} isLoading={isLoading || !meta || !programId || !account || gasless.isLoading}>
      {children}
    </Button>
  );
}
