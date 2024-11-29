import { useNavigate } from 'react-router-dom';
import { useAccount } from '@gear-js/react-hooks';
import { EzTransactionsSwitch } from '@dapps-frontend/ez-transactions';
import { Button } from '@/ui';
import { START } from '@/App.routes';
import { Welcome } from '@/features/Main/components';
import styles from './Layout.module.scss';
import { cx } from '@/utils';
import { useAccountAvailableBalance } from '@/features/Wallet/hooks';
import { SIGNLESS_ALLOWED_ACTIONS } from '@/consts';
import { useGameQuery } from '@/features/Game/sails';

function Layout() {
  const navigate = useNavigate();

  const { game, isFetching } = useGameQuery();
  const { account } = useAccount();
  const { isAvailableBalanceReady, availableBalance } = useAccountAvailableBalance();

  const handleGoToPlay = async () => {
    if (isAvailableBalanceReady && account?.decodedAddress) {
      navigate(START, { replace: true });
    }
  };

  return (
    <Welcome>
      <Button
        label={game ? 'Continue Game' : 'Start the game'}
        variant="primary"
        size="large"
        onClick={handleGoToPlay}
        className={cx(styles['game-button'])}
        isLoading={!availableBalance?.value || !account?.decodedAddress || isFetching}
      />
      <EzTransactionsSwitch allowedActions={SIGNLESS_ALLOWED_ACTIONS} />
    </Welcome>
  );
}

export { Layout };
