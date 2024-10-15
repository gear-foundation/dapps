import { ReactComponent as VaraSVG } from '@/assets/images/icons/vara-coin.svg';
import { ReactComponent as TVaraSVG } from '@/assets/images/icons/tvara-coin.svg';
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
