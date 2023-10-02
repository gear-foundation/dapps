type Node = {
  address: string;
  isCustom: boolean;
  icon?: string;
};

type NodeSection = {
  caption: string;
  nodes: Node[];
};

type ResultNode = {
  caption: string;
  address: string;
};

export type { Node, NodeSection, ResultNode };
