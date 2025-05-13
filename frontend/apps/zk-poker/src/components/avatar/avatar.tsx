import clsx from 'clsx';

import { DefaultAvatar } from '@/assets/images';

import styles from './avatar.module.scss';

type Props = {
  avatar?: string;
  size?: 'sm' | 'md' | 'lg';
};

const Avatar = ({ avatar = DefaultAvatar, size = 'md' }: Props) => {
  return (
    <div className={clsx(styles.avatar, styles[size])}>
      <img src={avatar} alt="avatar" className={styles.image} />
    </div>
  );
};

export { Avatar };
