import './index.css';
import './App.scss';
import { useApi, useAccount } from '@gear-js/react-hooks';
import { Footer, Header } from 'components/layout';
import { ApiLoader } from 'components/loaders/api-loader';
import { withProviders } from 'app/hocs';
import { Home } from './pages/home';

const Component = () => {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();
  return (
    <div className="flex flex-col min-h-screen">
      <Header />
      <main className="flex flex-col flex-1">{isApiReady && isAccountReady ? <Home /> : <ApiLoader />}</main>
      <Footer />
    </div>
  );
};

export const App = withProviders(Component);
