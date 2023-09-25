import { ReactElement } from 'react';

export interface GalleryCollectionProps {
  title: string;
  data: { id: string; component: ReactElement }[];
  switchMenu?: {
    name: string;
    value: string;
    activeByDefault?: boolean;
    onSelect?: () => void;
  }[];
  filterOptions?: {
    [key: string]: {
      label: string;
      value: string;
      onSelect?: () => void;
    };
  };
  emptyText?: ReactElement;
}
