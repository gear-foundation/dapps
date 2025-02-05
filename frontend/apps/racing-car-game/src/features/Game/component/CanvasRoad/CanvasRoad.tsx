import { LegacyRef, Ref, memo, useCallback, useEffect, useRef } from 'react';
import isEqual from 'lodash.isequal';
import styles from './CanvasRoad.module.scss';
import { cx } from '@/utils';
import startSVG from '@/assets/icons/game-start-icon.svg';
import finishSVG from '@/assets/icons/game-finish-icon.svg';
import roadLineSVG from '@/assets/icons/road-line-svg.svg';
import sectionEndLineSVG from '@/assets/icons/section-end-line.svg';

import PlayerCarSVG from '@/assets/icons/player-car-icon.svg?react';
import ContractCarSVG from '@/assets/icons/contract-car-icon.svg?react';

import smokeGIF from '@/assets/icons/smoke.gif';
import fireGIF from '@/assets/icons/fire.gif';
import speedGIF from '@/assets/icons/gif-speed.gif';
import { CanvasRoadProps } from './CanvasRoad.interface';

function CanvasRoadComponent({ cars, carIds, imagesCollection }: CanvasRoadProps) {
  const roadDistanceToStart = 300;

  const canvasRoadRef: LegacyRef<HTMLCanvasElement> = useRef(null);
  const roadRef: Ref<HTMLDivElement> = useRef(null);

  const drawSegment = (ctx: CanvasRenderingContext2D, segmentNumber: number, roadStart: number) => {
    const stripes = imagesCollection[roadLineSVG];
    const sectionEndLine = imagesCollection[sectionEndLineSVG];
    const segmentWidth = 156;
    const segmentHeight = 264;
    const renderStart = segmentNumber * segmentWidth + roadStart;

    ctx.fillStyle = 'transparent';

    ctx.fillRect(renderStart, 0, segmentWidth, segmentHeight);

    ctx.drawImage(stripes, renderStart, (segmentHeight * 1) / 3);
    ctx.drawImage(stripes, renderStart, (segmentHeight * 2) / 3);

    ctx.drawImage(sectionEndLine, renderStart + segmentWidth, 11);
    ctx.drawImage(sectionEndLine, renderStart + segmentWidth, segmentHeight - 10 - sectionEndLine.height);

    ctx.fillStyle = '#C5C5C5';

    ctx.fillText(String(segmentNumber + 1), renderStart + segmentWidth - 20, 22);
    ctx.fillText(String(segmentNumber + 1), renderStart + segmentWidth - 20, segmentHeight - 14);
  };

  const initRoad = useCallback(() => {
    const roadCtx = canvasRoadRef.current?.getContext('2d');
    const sections = Array(20)
      .fill(0)
      .map((_, i) => i);

    if (roadCtx) {
      const startLine = imagesCollection[startSVG];
      const finishLine = imagesCollection[finishSVG];

      roadCtx.drawImage(startLine, roadDistanceToStart, (264 - startLine.height) / 2);

      sections.forEach((item) => {
        drawSegment(roadCtx, item, roadDistanceToStart + startLine.width + 15);
      });

      roadCtx.drawImage(
        finishLine,
        roadDistanceToStart + startLine.width + 30 + sections.length * 156,
        (264 - startLine.height) / 2,
      );
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [imagesCollection]);

  const correctFocusOnPlayerCar = useCallback(() => {
    const roadCtx = canvasRoadRef.current?.getContext('2d');

    if (roadCtx && cars) {
      if (cars?.[carIds[0]]) {
        if (cars?.[carIds[0]].position > window.screen.width / 2) {
          roadRef.current?.scrollTo({ left: cars[carIds[0]].position - window.screen.width / 2, behavior: 'smooth' });
        } else {
          roadRef.current?.scrollTo({ left: 0 });
        }
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
      <canvas ref={canvasRoadRef} width={4000} height={264} className={cx(styles['road-canvas'])} />
      {cars && (
        <>
          {carIds.map((id, i) => (
            <div
              key={id}
              style={{
                transform: `translateX(${cars[id].position}px)`,
                top: `${cars[id].positionY}px`,
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

const CanvasRoad = memo(CanvasRoadComponent, isEqual);

export { CanvasRoad };
