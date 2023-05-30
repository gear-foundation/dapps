import { useEffect, useState } from 'react';
import { useAtom } from 'jotai';
import { useAlert } from '@gear-js/react-hooks';
import { ADDRESS, LOCAL_STORAGE } from 'consts';
import { NodeSection } from './types';
import { concatNodes, isDevSection, getLocalNodes, getLocalNodesFromLS } from './utils';
import { DEVELOPMENT_SECTION, NODE_ADDRESS_ATOM } from './consts';

function useNodeAddress() {
  const [address] = useAtom(NODE_ADDRESS_ATOM);

  return address;
}

function useNodes() {
  const alert = useAlert();

  const [nodeSections, setNodeSections] = useState<NodeSection[]>([]);
  const [isNodesLoading, setIsNodesLoading] = useState(true);

  const addNode = (address: string) => {
    const newLocalNode = { isCustom: true, address };

    const allNodes = concatNodes(nodeSections, newLocalNode);

    const devSection = allNodes.find(isDevSection);
    const localNodes = devSection ? getLocalNodes(devSection.nodes) : [newLocalNode];

    setNodeSections(allNodes);

    localStorage.setItem(LOCAL_STORAGE.NODES, JSON.stringify(localNodes));
  };

  const removeNode = (address: string) =>
    setNodeSections((prevState) =>
      prevState.map((section) => {
        if (isDevSection(section)) {
          const filtredNodes = section.nodes.filter((node) => node.address !== address);

          localStorage.setItem(LOCAL_STORAGE.NODES, JSON.stringify(filtredNodes.filter(({ isCustom }) => isCustom)));

          return { caption: section.caption, nodes: filtredNodes };
        }

        return section;
      }),
    );

  useEffect(() => {
    fetch(ADDRESS.DEFAULT_NODES)
      .then((response) => response.json())
      .then((sections) => {
        const localNodes = getLocalNodesFromLS();

        const isDevSectionExist = sections.find(isDevSection);

        const allNodes = isDevSectionExist
          ? concatNodes(sections, localNodes)
          : sections.concat({ caption: DEVELOPMENT_SECTION, nodes: localNodes });

        setNodeSections(allNodes);
      })
      .catch((error) => alert.error(error.message))
      .finally(() => setIsNodesLoading(false));

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return { nodeSections, isNodesLoading, addNode, removeNode };
}

export { useNodes, useNodeAddress };
