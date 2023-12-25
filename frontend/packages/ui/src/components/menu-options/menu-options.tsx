import { useAccount } from '@gear-js/react-hooks';
import styles from './menu-options.module.css';
import { ReactComponent as UserSVG } from './assets/user.svg';
import { ReactComponent as GridSVG } from './assets/grid.svg';
import clsx from 'clsx';

type Props = {
  customItems?: {
    icon?: React.FunctionComponent<React.SVGProps<SVGSVGElement>>;
    option: JSX.Element;
  }[];
};

export function MenuOptions({ customItems }: Props) {
  const { account } = useAccount();

  return (
    <div className={clsx(styles.container)}>
      {customItems?.map(({ icon: Icon, option }) => (
        <div className={clsx(styles.item)}>
          {Icon ? <Icon /> : null}
          {option}
        </div>
      ))}
      <hr />
      <a
        href={`https://vara.subscan.io/account/${account?.address}`}
        target="_blank"
        rel="noreferrer"
        className={clsx(styles.item)}>
        <UserSVG className={clsx(styles['user-svg'])} />
        <span>View in Blockchain Explorer</span>
      </a>
      <a href="https://vara.network/ecosystem" target="_blank" rel="noreferrer" className={clsx(styles.item)}>
        <GridSVG className={clsx(styles['user-svg'])} />
        <span>View other projects on Vara</span>
      </a>
      <hr />
    </div>
  );
}
