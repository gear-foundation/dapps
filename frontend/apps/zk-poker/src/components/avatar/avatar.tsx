import { HexString } from '@gear-js/api';
import clsx from 'clsx';

import {
  DefaultAvatar0,
  DefaultAvatar1,
  DefaultAvatar2,
  DefaultAvatar3,
  DefaultAvatar4,
  DefaultAvatar5,
  DefaultAvatar6,
  DefaultAvatar7,
} from '@/assets/images';

import styles from './avatar.module.scss';

type Props = {
  avatar?: string;
  size?: 'sm' | 'md' | 'lg';
  className?: string;
  isEmpty?: boolean;
  isHidden?: boolean;
  address?: HexString;
};

const defaultAvatars = [
  DefaultAvatar0,
  DefaultAvatar1,
  DefaultAvatar2,
  DefaultAvatar3,
  DefaultAvatar4,
  DefaultAvatar5,
  DefaultAvatar6,
  DefaultAvatar7,
];

const Avatar = ({ avatar, size = 'md', className, isEmpty, isHidden, address }: Props) => {
  const getAvatarByAddress = (_address: HexString) => {
    const hex = _address.slice(-2);
    const index = parseInt(hex, 16) % defaultAvatars.length;
    return defaultAvatars[index];
  };

  const displayAvatar = address ? getAvatarByAddress(address) : avatar || DefaultAvatar0;

  return (
    <div className={clsx(styles[size], isEmpty && styles.empty, isHidden && styles.hidden, className)}>
      {!isEmpty && displayAvatar && <img src={displayAvatar} alt="avatar" className={styles.image} />}
    </div>
  );
};

export { Avatar };
