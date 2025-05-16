import clsx from 'clsx';

import { DefaultAvatar } from '@/assets/images';

import styles from './avatar.module.scss';

type Props = {
  avatar?: string;
  size?: 'sm' | 'md' | 'lg';
  className?: string;
};

const Avatar = ({ avatar = DefaultAvatar, size = 'md', className }: Props) => {
  return (
    <div className={clsx(styles[size], className)}>
      <img src={avatar} alt="avatar" className={styles.image} />
    </div>
  );
};

export { Avatar };
