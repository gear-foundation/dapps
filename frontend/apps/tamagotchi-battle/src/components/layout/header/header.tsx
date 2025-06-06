import { AccountComponent } from './account';
import { Logo } from './logo';

export const Header = () => (
  <header className="container flex justify-between items-center py-5 xxl:py-7.5">
    <Logo />
    <AccountComponent />
  </header>
);
