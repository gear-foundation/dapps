import { LegacyRef, Ref, memo, useCallback, useEffect, useRef } from 'react';
import isEqual from 'lodash.isequal';
import styles from './CanvasRoadMobile.module.scss';
import { cx } from '@/utils';
import startSVG from '@/assets/icons/game-start-icon-vertical.svg';
import finishSVG from '@/assets/icons/game-finish-icon-vertical.svg';
import roadLineSVG from '@/assets/icons/road-line-vertical.svg';
import sectionEndLineSVG from '@/assets/icons/section-end-line-vertical.svg';

import PlayerCarSVG from '@/assets/icons/player-car-icon.svg?react';
import ContractCarSVG from '@/assets/icons/contract-car-icon.svg?react';

import smokeGIF from '@/assets/icons/smoke.gif';
import fireGIF from '@/assets/icons/fire.gif';
import speedGIF from '@/assets/icons/gif-speed.gif';
import { CanvasRoadProps } from './CanvasRoadMobile.interface';

function CanvasRoadMobileComponent({ cars, carIds, imagesCollection }: CanvasRoadProps) {
  const roadDistanceToStart = 300;

  const canvasRoadRef: LegacyRef<HTMLCanvasElement> = useRef(null);
  const roadRef: Ref<HTMLDivElement> = useRef(null);

  const drawSegment = (ctx: CanvasRenderingContext2D, segmentNumber: number, roadStart: number) => {
    const segmentWidth = 264;
    const segmentHeight = 156;
    const stripes = imagesCollection[roadLineSVG];
    const sectionEndLine = imagesCollection[sectionEndLineSVG];

    const renderStart = 4000 - (segmentNumber * segmentHeight + roadStart);

    ctx.fillStyle = 'transparent';

    ctx.fillRect(0, renderStart, segmentWidth, segmentHeight);

    ctx.drawImage(stripes, (segmentWidth * 1) / 3, renderStart - stripes.height);
    ctx.drawImage(stripes, (segmentWidth * 2) / 3, renderStart - stripes.height);

    ctx.drawImage(sectionEndLine, 11, renderStart - segmentHeight);
    ctx.drawImage(sectionEndLine, segmentWidth - 10 - sectionEndLine.width, renderStart - segmentHeight);

    ctx.fillStyle = '#C5C5C5';

    ctx.fillText(String(segmentNumber + 1), 14, renderStart - segmentHeight + 20);
    ctx.fillText(String(segmentNumber + 1), segmentWidth - 12 - sectionEndLine.width, renderStart - segmentHeight + 20);
  };

  const initRoad = useCallback(() => {
    const roadCtx = canvasRoadRef.current?.getContext('2d');

    if (roadCtx) {
      const sections = Array(20)
        .fill(0)
        .map((_, i) => i);
      const startLine = imagesCollection[startSVG];
      const finishLine = imagesCollection[finishSVG];

      roadCtx.drawImage(startLine, (264 - startLine.width) / 2, 4000 - roadDistanceToStart - startLine.height);

      sections.forEach((item) => {
        drawSegment(roadCtx, item, roadDistanceToStart + startLine.height + 15);
      });

      roadCtx.drawImage(
        finishLine,
        (264 - finishLine.width) / 2,
        4000 - (roadDistanceToStart + startLine.height + 30 + sections.length * 156) - finishLine.height,
      );
    }

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [imagesCollection]);

  const correctFocusOnPlayerCar = useCallback(() => {
    const roadCtx = canvasRoadRef.current?.getContext('2d');

    if (roadCtx && cars) {
      if (cars?.[carIds[0]]) {
        roadRef.current?.scrollTo({
          top: 4000 - cars[carIds[0]].position - window.screen.height / 2 + 100,
          behavior: 'smooth',
        });
      }
    }
  }, [cars, carIds]);

  useEffect(() => {
    initRoad();
  }, [initRoad]);

  useEffect(() => {
    correctFocusOnPlayerCar();
  }, [correctFocusOnPlayerCar]);

  return (
    <div className={cx(styles.road)} ref={roadRef}>
      <canvas ref={canvasRoadRef} width={264} height={4000} className={cx(styles['road-canvas'])} />
      {cars && (
        <>
          {carIds.map((id, i) => (
            <div
              key={id}
              style={{
                transform: `translateY(${4000 - cars[id].position - 100}px) rotate(-90deg)`,
                left: `${cars[id].positionY - 34}px`,
              }}
              className={cx(styles.car)}>
              {cars[id].effect && (
                <>
                  {(cars[id].effect === 'accelerated' || cars[id].effect === 'sAndA') && (
                    <img
                      src={speedGIF}
                      alt="smoke"
                      className={cx(styles['car-effect'], styles[`car-effect-accelerated`])}
                    />
                  )}
                  {(cars[id].effect === 'shooted' || cars[id].effect === 'sAndA') && (
                    <div>
                      <img
                        src={fireGIF}
                        alt="smoke"
                        className={cx(styles['car-effect'], styles[`car-effect-shooted-fire`])}
                      />
                      <img
                        src={smokeGIF}
                        alt="smoke"
                        className={cx(styles['car-effect'], styles[`car-effect-shooted-smoke`])}
                      />
                    </div>
                  )}
                </>
              )}
              {i === 0 ? <PlayerCarSVG /> : <ContractCarSVG />}
            </div>
          ))}
        </>
      )}
    </div>
  );
}

const CanvasRoadMobile = memo(CanvasRoadMobileComponent, isEqual);

export { CanvasRoadMobile };
