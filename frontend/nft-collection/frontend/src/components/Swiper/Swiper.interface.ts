import { ReactElement } from 'react';

export interface SwiperProps {
  title?: string;
  data: ReactElement[];
  withNavigation?: boolean;
  titleClass?: string;
}
