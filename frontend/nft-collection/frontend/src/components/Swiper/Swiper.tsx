import { useState } from 'react';
import { useKeenSlider } from 'keen-slider/react.es';
import { SwiperProps } from './Swiper.interface';
import styles from './Swiper.module.scss';
import { useMediaQuery } from '@/hooks';
import { cx } from '@/utils';
import 'keen-slider/keen-slider.min.css';
import { Button } from '@/ui';
import leftArrow from '@/assets/icons/left.svg';
import rightArrow from '@/assets/icons/right.svg';

function Swiper({ title, data, withNavigation, titleClass }: SwiperProps) {
  const [currentIndex, setCurrentIndex] = useState(0);
  const isTablet = useMediaQuery(1200);
  const isSmallTablet = useMediaQuery(710);
  const isMobile = useMediaQuery(430);

  const handleDetectMedia = () => {
    if (isMobile) {
      return 1.5;
    }

    if (isSmallTablet) {
      return 2;
    }

    if (isTablet) {
      return 3;
    }

    return 4;
  };

  const [sliderRef, instanceRef] = useKeenSlider(
    {
      slides: {
        perView: handleDetectMedia(),
        spacing: 16,
      },
      slideChanged(slider) {
        setCurrentIndex(slider.track.details.rel);
      },
    },
    [
      // add plugins here
    ],
  );

  const handlePrevSlide = () => {
    instanceRef.current?.prev();
  };

  const handleNextSlide = () => {
    instanceRef.current?.next();
  };

  return (
    <div className={cx(styles.wrapper)}>
      <div className={cx(styles.header)}>
        {title && <h4 className={cx(styles.title, titleClass || '')}>{title}</h4>}
        {withNavigation && (
          <div className={cx(styles['nav-wrapper'])}>
            <Button variant="icon" icon={leftArrow} onClick={handlePrevSlide} disabled={currentIndex === 0} />
            <Button
              variant="icon"
              icon={rightArrow}
              onClick={handleNextSlide}
              disabled={currentIndex === data.length - Math.floor(handleDetectMedia())}
            />
          </div>
        )}
      </div>
      <div className={cx(styles['swiper-wrapper'])}>
        <div ref={sliderRef} className="keen-slider">
          {data.map((item) => (
            <div className="keen-slider__slide">{item}</div>
          ))}
        </div>
      </div>
    </div>
  );
}

export { Swiper };
