import { useApi, useAccount } from '@gear-js/react-hooks';
import { Footer } from 'ui';
import { Routing } from 'pages';
import { Header } from 'components/layout';
import { ApiLoader } from 'components/loaders/api-loader';
import { withProviders } from 'app/hocs';
import './index.css';
import './App.scss';

const Component = () => {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  return (
    <div className="flex flex-col min-h-screen">
      <Header />
      <main className="flex flex-col flex-1">{isApiReady && isAccountReady ? <Routing /> : <ApiLoader />}</main>
      <Footer />
    </div>
  );
};

export const App = withProviders(Component);
