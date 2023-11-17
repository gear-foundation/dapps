import { Fragment } from 'react';
import { TransitionGroup, CSSTransition } from 'react-transition-group';
import { GalleryProps } from './Gallery.interface';
import styles from './Gallery.module.scss';
import { cx } from '@/utils';
import 'swiper/css';
import 'swiper/css/navigation';

function Gallery({ data, emptyText }: GalleryProps) {
  return (
    <>
      {data.length ? (
        <TransitionGroup component={null}>
          <div className={cx(styles['gallery-wrapper'])}>
            {data.map((item) => (
              <CSSTransition
                key={item.id}
                timeout={500}
                classNames={{
                  enter: cx(styles['item-enter']),
                  enterActive: cx(styles['item-enter-active']),
                  exit: cx(styles['item-exit']),
                  exitActive: cx(styles['item-exit-active']),
                }}>
                {item.component}
              </CSSTransition>
            ))}
          </div>
        </TransitionGroup>
      ) : (
        <div className={cx(styles['empty-wrapper'])}>
          {new Array(4).fill(0).map(() => (
            <div className={cx(styles['empty-item'])} />
          ))}
          <div className={cx(styles['empty-text-wrapper'])}>
            <h5 className={cx(styles['empty-text-heading'])}>Nothing here yet</h5>
            <div className={cx(styles['empty-text-main'])}>
              {emptyText || (
                <>
                  <span>No items yet</span>
                  <span>Something is wrong</span>
                </>
              )}
            </div>
          </div>
        </div>
      )}
    </>
  );
}

export { Gallery };
