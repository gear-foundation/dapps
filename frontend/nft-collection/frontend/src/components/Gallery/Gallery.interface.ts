import { ReactElement } from 'react';

export interface GalleryProps {
  data: { id: string; component: ReactElement }[];
  emptyText?: JSX.Element;
}
