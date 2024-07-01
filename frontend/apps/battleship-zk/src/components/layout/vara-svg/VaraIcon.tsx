import { ReactComponent as VaraSVG } from '@/assets/images/icons/vara-coin.svg';
import { ReactComponent as TVaraSVG } from '@/assets/images/icons/tvara-coin.svg';
import { useAccountDeriveBalancesAll, useApi, useBalanceFormat } from '@gear-js/react-hooks';

function VaraIcon() {
  const { getFormattedBalance } = useBalanceFormat();
  const balances = useAccountDeriveBalancesAll();
  const { isApiReady } = useApi();

  const balance =
    isApiReady && balances?.freeBalance ? getFormattedBalance(balances.freeBalance.toString()) : undefined;

  return balance?.unit?.toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />;
}

export { VaraIcon };
