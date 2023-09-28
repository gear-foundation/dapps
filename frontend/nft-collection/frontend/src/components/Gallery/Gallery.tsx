import { motion } from 'framer-motion';
import { GalleryProps } from './Gallery.interface';
import styles from './Gallery.module.scss';
import { cx } from '@/utils';
import 'swiper/css';
import 'swiper/css/navigation';

function Gallery({ data, emptyText }: GalleryProps) {
  return (
    <>
      {data.length ? (
        <div className={cx(styles['gallery-wrapper'])}>
          {data.map((item) => (
            <motion.div
              className={styles.item}
              key={item.id}
              initial={{ opacity: 0 }}
              whileInView={{ opacity: 1 }}
              viewport={{ once: true }}>
              {item.component}
            </motion.div>
          ))}
        </div>
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
