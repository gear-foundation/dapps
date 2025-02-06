import VaraSVG from '@/assets/images/icons/vara-coin.svg?react';
import TVaraSVG from '@/assets/images/icons/tvara-coin.svg?react';
import { useApi } from '@gear-js/react-hooks';

function VaraIcon() {
  const { api } = useApi();

  return api?.registry.chainTokens[0].toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />;
}

export { VaraIcon };
