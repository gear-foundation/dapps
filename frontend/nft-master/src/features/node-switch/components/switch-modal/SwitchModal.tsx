import { Button } from '@gear-js/ui';
import { Modal } from 'components';
import { ADDRESS } from 'consts';
import { ReactComponent as PlusSVG } from '../../assets/plus.svg';
import { ReactComponent as SwitchSVG } from '../../assets/switch.svg';
import { NodeSection } from '../../types';
import { Node } from '../node';
import styles from './SwitchModal.module.scss';

type Props = {
  nodeSections: NodeSection[];
  selectedNode: string;
  selectNode: (address: string) => void;
  removeNode: (address: string) => void;
  onSwitch: () => void;
  onAdd: () => void;
  onClose: () => void;
};

function SwitchModal(props: Props) {
  const { nodeSections, selectedNode, selectNode, removeNode, onSwitch, onAdd, onClose } = props;

  const isCurrentNode = selectedNode === ADDRESS.NODE;

  const getNodes = (section: NodeSection) =>
    section.nodes.map((node) => (
      <Node
        key={node.address}
        address={node.address}
        isCustom={node.isCustom}
        icon={node.icon}
        selectedNode={selectedNode}
        selectNode={selectNode}
        removeLocalNode={removeNode}
      />
    ));

  const getNodeSections = () =>
    nodeSections.map((section) => (
      <li key={section.caption}>
        <h2 className={styles.caption}>{section.caption}</h2>
        <ul className={styles.section}>{getNodes(section)}</ul>
      </li>
    ));

  return (
    <Modal heading="Change Network" onClose={onClose}>
      <ul className={styles.list}>{getNodeSections()}</ul>

      <div className={styles.buttons}>
        <Button icon={SwitchSVG} text="Switch" disabled={isCurrentNode} onClick={onSwitch} />
        <Button icon={PlusSVG} color="secondary" onClick={onAdd} />
      </div>
    </Modal>
  );
}

export { SwitchModal };
