import { Modal } from 'components';
import { LOCAL_STORAGE, SEARCH_PARAMS } from 'consts';
import { useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { ReactComponent as PlusSVG } from '../../assets/plus.svg';
import { ReactComponent as SwitchSVG } from '../../assets/switch.svg';
import { ICON, NODE_ADRESS_URL_PARAM } from '../../consts';
import { NodeSection } from '../../types';
import { useNodeAddress } from '../../hooks';
import { Node } from '../Node';
import styles from './SwitchModal.module.scss';

type Props = {
  sections: NodeSection[];
  onRemove: (address: string) => void;
  onAdd: () => void;
  onClose: () => void;
};

function SwitchModal({ sections, onRemove, onAdd, onClose }: Props) {
  const { nodeAddress } = useNodeAddress();
  const [selectedNode, setSelectedNode] = useState(nodeAddress);
  const isCurrentNode = selectedNode === nodeAddress;
  const [searchParams, setSearchParams] = useSearchParams();

  const switchNode = () => {
    // remove param to update it during nodeApi init
    searchParams.delete(NODE_ADRESS_URL_PARAM);
    searchParams.delete(SEARCH_PARAMS.MASTER_CONTRACT_ID);
    setSearchParams(searchParams);

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
