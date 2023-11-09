import { Loader, Modal } from 'components';
import { useAlert, withoutCommas } from '@gear-js/react-hooks';
import { cx, logger } from '@/utils';
import varaIcon from '@/assets/icons/vara-coin-silver.png';
import playSVG from '@/assets/icons/play-icon.svg';
import cancelSVG from '@/assets/icons/cross-circle-icon.svg';
import playlistCrossedSVG from '@/assets/icons/playlist-crossed-icon.svg';
import { Button } from '@/ui';
import { SubscribeModalProps } from './SubscribeModal.interface';
import { useSubscribeToStreamMessage } from '../../hooks';
import styles from './SubscribeModal.module.scss';
import { useGetStreamMetadata } from '@/features/CreateStream/hooks';
import { useCheckBalance, useHandleCalculateGas } from '@/hooks';
import { ADDRESS } from '@/consts';

function SubscribeModal({ speakerId, type, onClose }: SubscribeModalProps) {
  const { meta, isMeta } = useGetStreamMetadata();
  const sendSubscribeMessage = useSubscribeToStreamMessage();
  const calculateGas = useHandleCalculateGas(ADDRESS.CONTRACT, meta);
  const { checkBalance } = useCheckBalance();
  const alert = useAlert();

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

      calculateGas(payload)
        .then((res) => res.toHuman())
        .then(({ min_limit }) => {
          const minLimit = withoutCommas(min_limit as string);
          const gasLimit = Math.floor(Number(minLimit) + Number(minLimit) * 0.2);
          logger(`Calculating gas:`);
          logger(`MIN_LIMIT ${min_limit}`);
          logger(`LIMIT ${gasLimit}`);
          logger(`Calculated gas SUCCESS`);
          logger(`Sending message`);

          checkBalance(
            gasLimit,
            () =>
              sendSubscribeMessage({
                payload,
                gasLimit,
                onError: () => {
                  logger(`Errror send message`);
                },
                onSuccess: (messageId) => {
                  logger(`sucess on ID: ${messageId}`);
                  window.location.reload();
                },
                onInBlock: (messageId) => {
                  logger('messageInBlock');
                  logger(`messageID: ${messageId}`);
                },
              }),
            () => {
              logger(`Errror check balance`);
            },
          );
        })
        .catch((error) => {
          logger(error);
          alert.error('Gas calculation error');
        });

      handleCancelModal();
    }
  };

  return (
    <Modal heading={type === 'subscribe' ? 'Subscribe' : 'Unsubscribe'} onClose={onClose}>
      {isMeta ? (
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
      ) : (
        <Loader />
      )}
    </Modal>
  );
}

export { SubscribeModal };
