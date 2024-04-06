import { FunctionComponent, SVGProps } from 'react';

export type WalletItemProps = {
  Icon: FunctionComponent<SVGProps<SVGSVGElement> & { title?: string | undefined }>;
  name: string;
};
