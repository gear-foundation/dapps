import styles from './Layout.module.scss';
import { cx } from '@/utils';
import { TabPanel } from '@/ui';
import { ProfileInfo } from '../ProfileInfo';
import { tabs } from '../../config';
import { useGetStreamMetadata } from '@/features/CreateStream/hooks';
import { Loader } from '@/components';

function Layout() {
  const { isMeta } = useGetStreamMetadata();

  return isMeta ? (
    <div className={cx(styles.layout)}>
      <h1 className={cx(styles.title)}>My Account</h1>
      <div className={cx(styles.content)}>
        <ProfileInfo />
        <TabPanel tabs={tabs} />
      </div>
    </div>
  ) : (
    <Loader />
  );
}

export { Layout };
