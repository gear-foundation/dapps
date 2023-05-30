import { useState } from 'react';
import { ADDRESS, LOCAL_STORAGE } from 'consts';
import { useApi } from '@gear-js/react-hooks';
import { ReactComponent as OpenSVG } from '../../assets/open.svg';
import { useNodes } from '../../hooks';
import { SwitchModal } from '../switch-modal';
import styles from './NodeSwitch.module.scss';
import { AddModal } from '../add-modal';

function NodeSwitch() {
  const { api } = useApi();
  const { nodeSections, isNodesLoading, addLocalNode, removeLocalNode } = useNodes();

  const nodeAddress = ADDRESS.NODE;

  const [isSwitchModalOpen, setIsSwitchModalOpen] = useState(false);
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);
  const [selectedNode, setSelectedNode] = useState(nodeAddress);

  const openSwitchModal = () => setIsSwitchModalOpen(true);
  const closeSwitchModal = () => setIsSwitchModalOpen(false);

  const openAddModal = () => setIsAddModalOpen(true);
  const closeAddModal = () => setIsAddModalOpen(false);

  const switchNode = () => {
    localStorage.setItem(LOCAL_STORAGE.NODE, selectedNode);

    window.location.reload();
  };

  const chain = api ? api.runtimeChain.toString() : 'Loading...';

  return (
    <div>
      <button type="button" className={styles.button} onClick={openSwitchModal} disabled={isNodesLoading}>
        <span>{chain}</span> <OpenSVG />
      </button>

      {isSwitchModalOpen && (
        <SwitchModal
          nodeSections={nodeSections}
          selectedNode={selectedNode}
          selectNode={setSelectedNode}
          removeNode={removeLocalNode}
          onSwitch={switchNode}
          onAdd={openAddModal}
          onClose={closeSwitchModal}
        />
      )}

      {isAddModalOpen && <AddModal sections={nodeSections} onClose={closeAddModal} onSubmit={addLocalNode} />}
    </div>
  );
}

export { NodeSwitch };
