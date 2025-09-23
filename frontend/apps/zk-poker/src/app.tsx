import '@gear-js/vara-ui/dist/style-deprecated.css';
import { useAccount, useApi } from '@gear-js/react-hooks';

import { withProviders } from '@/app/hocs';
import { ApiLoader } from '@/components';
import { useAccountAvailableBalanceSync } from '@/features/wallet/hooks';

import { usePokerFactoryProgram } from './app/utils/sails';
import { Routing } from './pages';
import './app.scss';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const program = usePokerFactoryProgram();

  useAccountAvailableBalanceSync();

  const isAppReady = isApiReady && isAccountReady && program;

  return <main>{isAppReady ? <Routing /> : <ApiLoader />}</main>;
}

export const App = withProviders(Component);
