import { Button, radioStyles } from '@gear-js/ui';
import clsx from 'clsx';
import { SVGComponent } from 'types';
import { copyToClipboard } from 'utils';
import { ReactComponent as CopySVG } from '@/assets/icons/binary-code.svg';
import { ReactComponent as TrashSVG } from '@/assets/icons/trash.svg';
import styles from './Node.module.scss';

type Props = {
  address: string;
  isChecked: boolean;
  isActive: boolean;
  isCustom: boolean;
  SVG: SVGComponent;
  onChange: (value: string) => void;
  onRemove: (value: string) => void;
};

function Node(props: Props) {
  const { address, isChecked, isActive, isCustom, SVG, onChange, onRemove } = props;

  const labelClassName = clsx(styles.radio, isActive && styles.current);
  const radioClassName = clsx(radioStyles.input, styles.input);

  const handleChange = () => onChange(address);
  const handleCopy = () => copyToClipboard(address);
  const handleRemove = () => !isActive && onRemove(address);

  return (
    <li className={styles.node}>
      {/* eslint-disable-next-line jsx-a11y/label-has-associated-control */}
      <label className={labelClassName}>
        <input type="radio" name="node" checked={isChecked} onChange={handleChange} className={radioClassName} />

        <SVG className={styles.icon} />

        {address}
      </label>

      <div className={styles.buttons}>
        <Button
          icon={CopySVG}
          color="transparent"
          className={styles.copyButton}
          aria-label="Copy node address"
          onClick={handleCopy}
        />

        {isCustom && (
          <Button
            icon={TrashSVG}
            color="transparent"
            disabled={isActive}
            aria-label="Remove node address"
            onClick={handleRemove}
          />
        )}
      </div>
    </li>
  );
}

export { Node };
