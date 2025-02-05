import { InputProps } from '@gear-js/vara-ui';
import clsx from 'clsx';
import SearchSVG from '@/assets/images/icons/search.svg?react';
import styles from './Input.module.scss';

function Input(props: Omit<InputProps, 'size' | 'color'>) {
  const { className, ...attrs } = props;
  const wrapperClassName = clsx(styles.wrapper, className);

  return (
    <div className={wrapperClassName}>
      <SearchSVG />

      {/* eslint-disable-next-line react/jsx-props-no-spreading */}
      <input type="text" placeholder="Search" id="search" {...attrs} />
    </div>
  );
}

export { Input };
