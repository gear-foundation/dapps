import { MutableRefObject, memo, useEffect, useRef, useState } from 'react';
import isEqual from 'lodash.isequal';
import styles from './Road.module.scss';
import { cx, withoutCommas } from '@/utils';
import startSVG from '@/assets/icons/game-start-icon.svg';
import finishSVG from '@/assets/icons/game-finish-icon.svg';
import roadLineSVG from '@/assets/icons/road-line-svg.svg';
import sectionEndLineSVG from '@/assets/icons/section-end-line.svg';
import startVerticalSVG from '@/assets/icons/game-start-icon-vertical.svg';
import finishVerticalSVG from '@/assets/icons/game-finish-icon-vertical.svg';
import roadLineVerticalSVG from '@/assets/icons/road-line-vertical.svg';
import sectionEndLineVerticalSVG from '@/assets/icons/section-end-line-vertical.svg';
import { CarEffect, CarsState, RoadProps } from './Road.interface';
import { Cars } from '@/types';
import { Loader } from '@/components';
import { useMediaQuery } from '@/hooks';
import { CanvasRoad } from '../CanvasRoad';
import { CanvasRoadMobile } from '../CanvasRoadMobile';

function RoadComponent({ newCars, carIds, onRoadLoaded }: RoadProps) {
  const isMobile = useMediaQuery(768);
  const carDistanceFromInit = 160;

  const [cars, setCars] = useState<CarsState | null>(null);
  const [isRoadAssetsLoaded, setIsRoadAssetsLoaded] = useState<boolean>(false);

  const imagesCollection: MutableRefObject<Record<string, HTMLImageElement>> = useRef({});

  const loadImageSync = (src: string) =>
    new Promise((resolve, reject) => {
      const img = new Image();
      img.onload = () => resolve(img);
      img.onerror = () => reject();
      img.src = src;
      imagesCollection.current[src] = img;
    });

  const loadRoadAssets = async () => {
    await loadImageSync(startSVG);
    await loadImageSync(finishSVG);
    await loadImageSync(roadLineSVG);
    await loadImageSync(sectionEndLineSVG);
    await loadImageSync(startVerticalSVG);
    await loadImageSync(finishVerticalSVG);
    await loadImageSync(roadLineVerticalSVG);
    await loadImageSync(sectionEndLineVerticalSVG);

    setIsRoadAssetsLoaded(true);
    onRoadLoaded();
  };

  useEffect(() => {
    loadRoadAssets();

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const defineCarEffect = (roundResult: string | null): CarEffect => {
    if (roundResult) {
      if (roundResult === 'SlowedDown') {
        return 'shooted';
      }

      if (roundResult === 'Accelerated') {
        return 'accelerated';
      }

      if (roundResult === 'SlowedDownAndAccelerated') {
        return 'sAndA';
      }
    }

    return null;
  };

  const initCars = () => {
    const carPositionsY = [108, 30, 185];

    const carsToState: CarsState = carIds?.reduce(
      (acc, id, i) => ({
        ...acc,
        [id]: {
          ...newCars[id],
          speed: Number(withoutCommas(newCars[id].speed)),
          position: Number(withoutCommas(newCars[id].position)) + carDistanceFromInit,
          positionY: carPositionsY[i],
          effect: defineCarEffect(newCars[id].roundResult),
        },
      }),
      {},
    );

    setCars(carsToState);
  };

  const updateCars = (newCarsToUpdate: Cars) => {
    carIds.forEach((id) => {
      setCars((prev) =>
        prev
          ? {
              ...prev,
              [id]: {
                ...prev[id],
                speed: Number(withoutCommas(newCars[id].speed)),
                position: Number(withoutCommas(newCarsToUpdate[id].position)) + carDistanceFromInit,
                effect: defineCarEffect(newCarsToUpdate[id].roundResult),
              },
            }
          : null,
      );
    });
  };

  useEffect(() => {
    if (isRoadAssetsLoaded) {
      if (!cars) {
        initCars();
      }

      if (cars) {
        updateCars(newCars);
      }
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [newCars, isRoadAssetsLoaded]);

  return (
    <div className={cx(styles['road-container'], !isRoadAssetsLoaded ? styles['full-height'] : '')}>
      {isRoadAssetsLoaded ? (
        <>
          {isMobile ? (
            <CanvasRoadMobile cars={cars} carIds={carIds} imagesCollection={imagesCollection.current} />
          ) : (
            <CanvasRoad cars={cars} carIds={carIds} imagesCollection={imagesCollection.current} />
          )}
        </>
      ) : (
        <Loader />
      )}
    </div>
  );
}

const Road = memo(RoadComponent, isEqual);

export { Road };
