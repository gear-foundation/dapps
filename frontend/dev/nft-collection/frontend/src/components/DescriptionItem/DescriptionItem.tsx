import { cx } from '@/utils';
import styles from './DescriptionItem.module.scss';
import { DescriptionItemProps } from './DescriptionItem.interfaces';

function DescriptionItem({ icon, text }: DescriptionItemProps) {
  return (
    <div className={cx(styles.item)}>
      {icon}
      <span className={cx(styles['item-text'])}>{text}</span>
    </div>
  );
}

export { DescriptionItem };
