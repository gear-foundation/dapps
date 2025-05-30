import clsx from 'clsx';

import { DefaultAvatar } from '@/assets/images';

import styles from './avatar.module.scss';

type Props = {
  avatar?: string;
  size?: 'sm' | 'md' | 'lg';
  className?: string;
  isEmpty?: boolean;
  isHidden?: boolean;
};

const Avatar = ({ avatar = DefaultAvatar, size = 'md', className, isEmpty, isHidden }: Props) => {
  return (
    <div className={clsx(styles[size], isEmpty && styles.empty, isHidden && styles.hidden, className)}>
      {!isEmpty && <img src={avatar} alt="avatar" className={styles.image} />}
    </div>
  );
};

export { Avatar };
