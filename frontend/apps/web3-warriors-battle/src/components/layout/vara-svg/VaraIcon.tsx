import VaraSVG from '@/assets/images/icons/vara-coin.svg?react';
import TVaraSVG from '@/assets/images/icons/tvara-coin.svg?react';
import { useApi } from '@gear-js/react-hooks';

type VaraIconProps = {
  className?: string;
};

function VaraIcon({ className }: VaraIconProps) {
  const { api } = useApi();

  return api?.registry.chainTokens[0].toLowerCase() === 'vara' ? (
    <VaraSVG className={className} />
  ) : (
    <TVaraSVG className={className} />
  );
}

export { VaraIcon };
