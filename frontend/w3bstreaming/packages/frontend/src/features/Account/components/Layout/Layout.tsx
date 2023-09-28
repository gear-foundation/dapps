import styles from './Layout.module.scss';
import { cx } from '@/utils';
import { TabPanel } from '@/ui';
import { ProfileInfo } from '../ProfileInfo';
import { tabs } from '../../config';

function Layout() {
  return (
    <div className={cx(styles.layout)}>
      <h1 className={cx(styles.title)}>My Account</h1>
      <div className={cx(styles.content)}>
        <ProfileInfo />
        <TabPanel tabs={tabs} />
      </div>
    </div>
  );
}

export { Layout };
