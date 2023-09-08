import { useEffect, useState } from 'react';
import { useLocation, useSearchParams } from 'react-router-dom';
import { atom, useAtom } from 'jotai';
import { useAlert } from '@gear-js/react-hooks';
import { ADDRESS, LOCAL_STORAGE, SEARCH_PARAMS } from 'consts';
import { useContractAddress } from 'features/contract-address';
import { NodeSection } from './types';
import { concatNodes, isDevSection, getLocalNodes, getLocalNodesFromLS, getNodeAddressFromUrl } from './utils';
import { DEVELOPMENT_SECTION, NODE_ADRESS_URL_PARAM } from './consts';

const addressAtom = atom(
  getNodeAddressFromUrl() || (localStorage[LOCAL_STORAGE.NODE] as string) || ADDRESS.DETAULT_NODE,
);

function useNodeAddress() {
  const [nodeAddress] = useAtom(addressAtom);

  const isTestnet = nodeAddress === 'wss://vit.vara-network.io';

  const getIpfsAddress = (cid: string) =>
    isTestnet ? `${ADDRESS.TESTNET_IPFS_GATEWAY}/${cid}` : `${ADDRESS.IPFS_GATEWAY}/${cid}`;

  const getImageUrl = (value: string) => (value.startsWith('https://') ? value : getIpfsAddress(value));

  return { nodeAddress, isTestnet, getIpfsAddress, getImageUrl };
}

function useSearchParamsSetup() {
  const { contractAddress } = useContractAddress();
  const { nodeAddress } = useNodeAddress();

  const { pathname } = useLocation();
  const [searchParams, setSearchParams] = useSearchParams();

  useEffect(() => {
    searchParams.set(NODE_ADRESS_URL_PARAM, nodeAddress);
    searchParams.set(SEARCH_PARAMS.MASTER_CONTRACT_ID, contractAddress);

    setSearchParams(searchParams, { replace: true });

    // looking for pathname, cuz searchParams is not enough in case of page's <Navigate />
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [searchParams, pathname, nodeAddress, contractAddress]);
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

export { useNodes, useNodeAddress, useSearchParamsSetup };
