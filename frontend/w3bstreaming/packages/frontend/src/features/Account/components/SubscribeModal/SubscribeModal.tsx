import { Modal } from 'components';
import { cx } from '@/utils';
import varaIcon from '@/assets/icons/vara-coin-silver.png';
import playSVG from '@/assets/icons/play-icon.svg';
import cancelSVG from '@/assets/icons/cross-circle-icon.svg';
import playlistCrossedSVG from '@/assets/icons/playlist-crossed-icon.svg';
import { Button } from '@/ui';
import { SubscribeModalProps } from './SubscribeModal.interface';
import { useSubscribeToStreamMessage } from '../../hooks';
import styles from './SubscribeModal.module.scss';

function SubscribeModal({ speakerId, type, onClose }: SubscribeModalProps) {
  const sendSubscribeMessage = useSubscribeToStreamMessage();

  const handleCancelModal = () => {
    onClose();
  };

  const handleUnsubscribe = () => {
    if (speakerId) {
      const payload = {
        Unsubscribe: {
          account_id: speakerId,
        },
      };

      handleCancelModal();
    }
  };

  const handleSubscribe = () => {
    if (speakerId) {
      const payload = {
        Subscribe: {
          account_id: speakerId,
        },
      };

      handleCancelModal();

      sendSubscribeMessage(payload, {
        onSuccess: () => {
          window.location.reload();
        },
        onError: () => {
          console.log('error');
        },
      });
    }
  };

  return (
    <Modal heading={type === 'subscribe' ? 'Subscribe' : 'Unsubscribe'} onClose={onClose}>
      <div className={cx(styles.container)}>
        <p className={cx(styles.description)}>Are you sure you want to {type} from this streamer?</p>

        {type === 'subscribe' && (
          <div className={cx(styles['cont-per-month'])}>
            <span className={cx(styles['cont-per-month-label'])}>Per month:</span>
            <img src={varaIcon} alt="vara" className={cx(styles['cont-per-month-vara'])} />
            <span className={cx(styles['cont-per-month-value'])}>1</span>
            <span className={cx(styles['cont-per-month-currency'])}>vara</span>
          </div>
        )}

        <div className={cx(styles.controls)}>
          {type === 'subscribe' && (
            <Button variant="primary" label="Subscribe" icon={playSVG} onClick={handleSubscribe} />
          )}
          {type === 'unsubscribe' && (
            <Button
              variant="primary"
              label="Unsubscribe"
              icon={playlistCrossedSVG}
              onClick={handleUnsubscribe}
              className={cx(styles['unsubscribe-button'])}
            />
          )}
          <Button variant="text" label="Cancel" icon={cancelSVG} onClick={handleCancelModal} />
        </div>
      </div>
    </Modal>
  );
}

export { SubscribeModal };
