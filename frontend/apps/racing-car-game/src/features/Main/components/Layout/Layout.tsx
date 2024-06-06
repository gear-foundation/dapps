import { useAtomValue } from 'jotai';
import { useNavigate } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { EzTransactionsSwitch } from '@dapps-frontend/ez-transactions';
import { Button } from '@/ui';
import { CURRENT_GAME } from '@/atoms';
import { START } from '@/App.routes';
import { Welcome } from '@/features/Main/components';
import styles from './Layout.module.scss';
import { cx } from '@/utils';
import metaTxt from '@/assets/meta/meta.txt';
import { useProgramMetadata } from '@/hooks';
import { useAccountAvailableBalance } from '@/features/Wallet/hooks';
import { IS_STATE_READ_ATOM } from '@/features/Game/atoms';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/consts';

function Layout() {
  const navigate = useNavigate();
  const currentGame = useAtomValue(CURRENT_GAME);
  const isStateRead = useAtomValue(IS_STATE_READ_ATOM);
  const { account } = useAccount();
  const meta = useProgramMetadata(metaTxt);
  const { isAvailableBalanceReady, availableBalance } = useAccountAvailableBalance();

  const handleGoToPlay = async () => {
    if (isAvailableBalanceReady && account?.decodedAddress && meta) {
      navigate(START, { replace: true });
    }
  };

  return (
    <Welcome>
      <Button
        label={currentGame ? 'Continue Game' : 'Start the game'}
        variant="primary"
        size="large"
        onClick={handleGoToPlay}
        className={cx(styles['game-button'])}
        isLoading={!meta || !availableBalance?.value || !account?.decodedAddress || !isStateRead}
      />
      <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />
    </Welcome>
  );
}

export { Layout };
