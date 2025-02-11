import { useAccount } from '@gear-js/react-hooks';
import { ReactNode } from 'react';

type Props = {
  children: ReactNode;
  fallback?: ReactNode;
};

function OnLogin({ children, fallback }: Props) {
  const { account } = useAccount();

  return <>{account ? children : fallback}</>;
}

export { OnLogin };
