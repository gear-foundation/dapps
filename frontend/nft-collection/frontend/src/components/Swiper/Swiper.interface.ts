import { ReactElement } from 'react';
import Swiper from 'swiper';
import { SwiperProps as ReactSwiperProps } from 'swiper/react';

export interface SwiperProps extends ReactSwiperProps {
  data: ReactElement[];
  withNavigation?: boolean;
}
