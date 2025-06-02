import streamDateIcon from '@/assets/icons/hexagon-icon.png';
import noUserAvatarImg from '@/assets/icons/no-avatar-user-img.png';
import noStreamPreviewImg from '@/assets/icons/no-stream-preview-img.png';
import { cx } from '@/utils';

import { StreamProps } from '../../types';

import styles from './StreamTeaser.module.scss';

function StreamTeaser({ title, start_time, description, img_link, broadcasterInfo }: StreamProps) {
  const date = new Date(Number(start_time));

  return (
    <div className={cx(styles.card)}>
      <div className={cx(styles['card-top'])}>
        <img className={cx(styles['card-top-image'])} src={img_link || noStreamPreviewImg} alt="" />
        <div className={cx(styles['card-top-blur'])} />
        <div className={cx(styles['card-top-date-container'])}>
          <div className={cx(styles['card-top-date'])}>
            <img className={cx(styles['card-top-date-image'])} src={streamDateIcon} alt="" />
            <div className={cx(styles['card-top-date-content'])}>
              <span className={cx(styles['card-top-date-day'])}>{date.getDate()}</span>
              <span className={cx(styles['card-top-date-month'])}>
                {date.toLocaleString('default', { month: 'short' })}
              </span>
            </div>
          </div>
        </div>
        <div className={cx(styles['card-top-speaker-container'])}>
          <div className={cx(styles['card-top-speaker'])}>
            <img
              className={cx(styles['card-top-speaker-photo'])}
              src={broadcasterInfo?.img_link || noUserAvatarImg}
              alt="speaker"
            />
            <div className={cx(styles['card-top-speaker-content'])}>
              <span className={cx(styles['card-top-speaker-name'])}>
                {broadcasterInfo?.name} {broadcasterInfo?.surname}
              </span>
              <span className={cx(styles['card-top-speaker-descr'])}>Speaker</span>
            </div>
          </div>
        </div>
      </div>
      <div className={cx(styles['card-bottom'])}>
        <h5 className={cx(styles['card-bottom-label'])}>{title}</h5>
        <p className={cx(styles['card-bottom-description'])}>{description}</p>
      </div>
    </div>
  );
}

export { StreamTeaser };
