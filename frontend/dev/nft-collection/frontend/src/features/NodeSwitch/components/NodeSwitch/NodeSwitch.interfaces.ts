import { FunctionComponent, ReactElement, SVGProps } from 'react';

export type Chain = {
  name: string;
  address: string;
  icon: FunctionComponent<SVGProps<SVGSVGElement> & { title?: string | undefined }>;
} | null;

export type NodeSwitchProps = {
  children: ReactElement;
  onChainChange: (newChain: Chain) => void;
};
