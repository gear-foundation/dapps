import { useEffect, useState } from 'react';
import { useApi } from '@gear-js/react-hooks';
import { ReactComponent as OpenSVG } from '../../assets/open.svg';
import { useNodeAddress, useNodes } from '../../hooks';
import { SwitchModal } from '../SwitchModal';
import styles from './NodeSwitch.module.scss';
import { AddModal } from '../AddModal';
import { Chain, NodeSwitchProps } from './NodeSwitch.interfaces';
import { cx } from '@/utils';
import { Node, NodeSection } from '../../types';
import { ICON } from '../../consts';

function NodeSwitch({ children, onChainChange }: NodeSwitchProps) {
  const { api } = useApi();
  const { nodeAddress } = useNodeAddress();
  const chain = api?.runtimeChain.toString();
  console.log(api?.runtimeChain);
  console.log(nodeAddress);
  console.log(chain);

  const { nodeSections, isNodesLoading, addNode, removeNode } = useNodes();

  const [isSwitchModalOpen, setIsSwitchModalOpen] = useState(false);
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);

  const openSwitchModal = () => setIsSwitchModalOpen(true);
  const closeSwitchModal = () => setIsSwitchModalOpen(false);

  const openAddModal = () => setIsAddModalOpen(true);
  const closeAddModal = () => setIsAddModalOpen(false);

  const handleChainChange = (newChain: Chain) => {
    onChainChange(newChain);
  };

  useEffect(() => {
    console.log(nodeSections);
    if (nodeSections?.length && nodeAddress) {
      console.log(nodeSections);
      console.log(chain);
      const activeSection = nodeSections.find((section) => section.caption === chain) as NodeSection;
      console.log(activeSection);
      const activeNode = activeSection?.nodes?.find((item) => item.address === nodeAddress) as Node;
      const iconKey = (activeNode?.icon as keyof typeof ICON) || 'gear';
      console.log(activeNode);
      const SVG = ICON[iconKey];

      handleChainChange({
        name: activeSection.caption,
        address: activeNode.address,
        icon: SVG,
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [chain, nodeSections, nodeAddress]);

  return (
    <>
      <button
        type="button"
        className={cx(styles['invisible-button'])}
        onClick={openSwitchModal}
        disabled={isNodesLoading}>
        {children}
      </button>

      {isSwitchModalOpen && (
        <SwitchModal sections={nodeSections} onRemove={removeNode} onAdd={openAddModal} onClose={closeSwitchModal} />
      )}

      {isAddModalOpen && <AddModal sections={nodeSections} onClose={closeAddModal} onSubmit={addNode} />}
    </>
  );
}

export { NodeSwitch };
