import clsx from 'clsx';
import styles from './GameDetails.module.scss';

type Props = {
  items: {
    name: string;
    value: JSX.Element;
  }[];
  className?: {
    container?: string;
    item?: string;
  };
};

function GameDetails({ items, className }: Props) {
  return (
    <div className={clsx(styles.info, className?.container)}>
      {items.map((item) => (
        <div key={item.name} className={clsx(styles.item, className?.item)}>
          <span className={styles.itemName}>{item.name}</span>
          <span className={styles.itemValue}>{item.value}</span>
        </div>
      ))}
    </div>
  );
}

export { GameDetails };
