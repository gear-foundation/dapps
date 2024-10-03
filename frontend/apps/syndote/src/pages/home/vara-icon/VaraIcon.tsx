import { ReactComponent as VaraSVG } from 'assets/images/icons/vara-coin.svg';
import { ReactComponent as TVaraSVG } from 'assets/images/icons/tvara-coin.svg';
import { useApi } from '@gear-js/react-hooks';

function VaraIcon() {
  const { api } = useApi();

  return api?.registry.chainTokens[0].toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />;
}

export { VaraIcon };
