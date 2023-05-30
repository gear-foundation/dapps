import { useState } from 'react';
import { useApi } from '@gear-js/react-hooks';
import { ReactComponent as OpenSVG } from '../../assets/open.svg';
import { useNodes } from '../../hooks';
import { SwitchModal } from '../switch-modal';
import styles from './NodeSwitch.module.scss';
import { AddModal } from '../add-modal';

function NodeSwitch() {
  const { api } = useApi();
  const chain = api?.runtimeChain.toString();

  const { nodeSections, isNodesLoading, addNode, removeNode } = useNodes();

  const [isSwitchModalOpen, setIsSwitchModalOpen] = useState(false);
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);

  const openSwitchModal = () => setIsSwitchModalOpen(true);
  const closeSwitchModal = () => setIsSwitchModalOpen(false);

  const openAddModal = () => setIsAddModalOpen(true);
  const closeAddModal = () => setIsAddModalOpen(false);

  return (
    <>
      <button type="button" className={styles.button} onClick={openSwitchModal} disabled={isNodesLoading}>
        <span>{chain || 'Loading...'}</span> <OpenSVG />
      </button>

      {isSwitchModalOpen && (
        <SwitchModal sections={nodeSections} onRemove={removeNode} onAdd={openAddModal} onClose={closeSwitchModal} />
      )}

      {isAddModalOpen && <AddModal sections={nodeSections} onClose={closeAddModal} onSubmit={addNode} />}
    </>
  );
}

export { NodeSwitch };
