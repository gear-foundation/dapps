import { useAccount } from '@gear-js/react-hooks';
import { useLayoutEffect } from 'react';
import { Navigate } from 'react-router-dom';
import { Welcome } from 'features/welcome';

function Home() {
  const { account } = useAccount();

  useLayoutEffect(() => {
    document.body.classList.add('welcome');

    return () => {
      document.body.classList.remove('welcome');
    };
  }, []);

  return account ? <Navigate to="/list" /> : <Welcome />;
}

export { Home };
