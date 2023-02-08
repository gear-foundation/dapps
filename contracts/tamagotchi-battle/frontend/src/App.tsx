import './index.css';
import './App.scss';
import { useApi, useAccount } from '@gear-js/react-hooks';
import { Footer, Header } from 'components/layout';
import { ApiLoader } from 'components/loaders/api-loader';
import { withProviders } from 'app/hocs';
import { Routing } from './pages';
import { useLocation } from 'react-router-dom';

const Component = () => {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  const { pathname } = useLocation();
  return (
    <div className="flex flex-col min-h-screen">
      <Header />
      <main className="flex flex-col flex-1 pt-3 pb-5">
        {isApiReady && isAccountReady ? <Routing /> : <ApiLoader />}
      </main>
      {pathname === '/' && <Footer />}
    </div>
  );
};

export const App = withProviders(Component);
