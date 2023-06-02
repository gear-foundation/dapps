import { Modal } from 'components';
import { LOCAL_STORAGE } from 'consts';
import { useEffect, useState } from 'react';
import { ReactComponent as PlusSVG } from '../../assets/plus.svg';
import { ReactComponent as SwitchSVG } from '../../assets/switch.svg';
import { ICON } from '../../consts';
import { NodeSection } from '../../types';
import { useNodeAddress } from '../../hooks';
import { Node } from '../node';
import styles from './SwitchModal.module.scss';

type Props = {
  sections: NodeSection[];
  onRemove: (address: string) => void;
  onAdd: () => void;
  onClose: () => void;
};

function SwitchModal(props: Props) {
  const { sections, onRemove, onAdd, onClose } = props;

  const nodeAddress = useNodeAddress();
  const [selectedNode, setSelectedNode] = useState(nodeAddress);
  const isCurrentNode = selectedNode === nodeAddress;

  const switchNode = () => {
    localStorage.setItem(LOCAL_STORAGE.NODE, selectedNode);
    window.location.reload();
  };

  const getNodes = (section: NodeSection) =>
    section.nodes.map(({ address, isCustom, icon }) => {
      const isChecked = address === selectedNode;
      const isActive = address === nodeAddress;

      const iconKey = (icon as keyof typeof ICON) || 'gear';
      const SVG = ICON[iconKey];

      return (
        <Node
          key={address}
          address={address}
          isChecked={isChecked}
          isActive={isActive}
          isCustom={isCustom}
          SVG={SVG}
          onChange={setSelectedNode}
          onRemove={onRemove}
        />
      );
    });

  const getSections = () =>
    sections.map((section) => (
      <li key={section.caption}>
        <h2 className={styles.caption}>{section.caption}</h2>
        <ul className={styles.section}>{getNodes(section)}</ul>
      </li>
    ));

  useEffect(() => {
    setSelectedNode(nodeAddress);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [sections]);

  return (
    <Modal heading="Change Network" onClose={onClose}>
      <ul className={styles.list}>{getSections()}</ul>

      <div className={styles.buttons}>
        <button type="button" disabled={isCurrentNode} onClick={switchNode} className={styles.switchButton}>
          <SwitchSVG /> <span>Switch</span>
        </button>

        <button type="button" onClick={onAdd} className={styles.addButton}>
          <PlusSVG />
        </button>
      </div>
    </Modal>
  );
}

export { SwitchModal };
