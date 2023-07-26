import { useAccount } from '@gear-js/react-hooks';
import { useLayoutEffect } from 'react';
import { Navigate } from 'react-router-dom';
import { Welcome } from 'features/welcome';

function Home() {
  const { account } = useAccount();

  useLayoutEffect(() => {
    if (account) return;

    document.body.classList.add('welcome');

    return () => {
      document.body.classList.remove('welcome');
    };
  }, [account]);

  return account ? <Navigate to="/list" replace /> : <Welcome />;
}

export { Home };
