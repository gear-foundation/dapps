import { Logo } from './logo';
import { AccountComponent } from './account';

export const Header = () => (
  <header className="container flex justify-between items-center py-5">
    <Logo />
    <AccountComponent />
  </header>
);
