import clsx from 'clsx';
import { Button } from '@gear-js/vara-ui';
import { BaseComponentProps } from '@/app/types';
import styles from './list.module.scss';
import { useState } from 'react';

type ListProps = BaseComponentProps & {
  items: React.ReactNode[];
  maxLength: number;
};

const List = ({ items, className, maxLength, ...restProps }: ListProps) => {
  const [showAll, setShowAll] = useState(false);

  const displayedItems = showAll ? items : items.slice(0, maxLength);

  return (
    <div className={clsx(styles.wrapper, className)} {...restProps}>
      {displayedItems}
      {items.length > maxLength && !showAll && (
        <Button color="light" text="Show More" onClick={() => setShowAll(true)} />
      )}
    </div>
  );
};

export { List };
