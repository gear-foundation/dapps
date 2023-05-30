import { LOCAL_STORAGE } from 'consts';
import { Node, NodeSection } from './types';
import { DEVELOPMENT_SECTION } from './consts';

const isDevSection = (section: NodeSection) => section.caption === DEVELOPMENT_SECTION;

const getLocalNodes = (nodes: Node[]): Node[] =>
  nodes.reduce((result, node) => {
    if (node.isCustom) result.push(node);

    return result;
  }, [] as Node[]);

const getLocalNodesFromLS = (): Node[] => {
  const nodes = localStorage[LOCAL_STORAGE.NODES];

  return nodes ? JSON.parse(nodes) : [];
};

const concatNodes = (nodeSections: NodeSection[], value: Node | Node[]) =>
  nodeSections.map((section) => {
    if (isDevSection(section)) {
      return {
        caption: section.caption,
        nodes: section.nodes.concat(value),
      };
    }

    return section;
  });

const isNodeAddressValid = (address: string) => {
  const nodeRegex = /(ws|wss):\/\/[\w-.]+/;

  return nodeRegex.test(address);
};

const isNodeExists = (sections: NodeSection[], address: string) => {
  const nodes = sections.flatMap((section) => section.nodes);

  return nodes.some((node) => node.address === address);
};

export { concatNodes, isDevSection, getLocalNodes, getLocalNodesFromLS, isNodeAddressValid, isNodeExists };
