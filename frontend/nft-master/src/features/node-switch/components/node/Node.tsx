import { Button, radioStyles } from '@gear-js/ui';
import clsx from 'clsx';
import { ADDRESS } from 'consts';
import { copyToClipBoard } from 'utils';
import { ReactComponent as CopySVG } from 'assets/images/icons/copy.svg';
import { ReactComponent as TrashSVG } from '../../assets/trash.svg';
import { ICON } from '../../consts';
import { Node as NodeType } from '../../types';
import styles from './Node.module.scss';

type Props = NodeType & {
  selectedNode: string;
  selectNode: (address: string) => void;
  removeLocalNode: (address: string) => void;
};

function Node(props: Props) {
  const { address, isCustom, selectedNode, selectNode, removeLocalNode, icon = 'gear' } = props;

  const SVG = ICON[icon as keyof typeof ICON].NETWORK;

  const isCurrentNode = ADDRESS.NODE === address;

  const handleCopy = () => copyToClipBoard(address);

  const handleChange = () => selectNode(address);

  const handleRemove = () => {
    if (isCurrentNode) return;

    removeLocalNode(address);
  };

  return (
    <li id={address} className={styles.node}>
      {/* eslint-disable-next-line jsx-a11y/label-has-associated-control */}
      <label className={clsx(styles.radio, isCurrentNode && styles.current)}>
        <input
          type="radio"
          name="node"
          checked={selectedNode === address}
          onChange={handleChange}
          className={clsx(radioStyles.input, styles.input)}
        />

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
            disabled={isCurrentNode}
            aria-label="Remove node address"
            onClick={handleRemove}
          />
        )}
      </div>
    </li>
  );
}

export { Node };
