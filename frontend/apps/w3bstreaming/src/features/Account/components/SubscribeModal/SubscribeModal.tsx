import { HexString } from '@gear-js/api';
import { useAccount } from '@gear-js/react-hooks';
import { useNavigate } from 'react-router-dom';

import { ACCOUNT } from '@/App.routes';
import { usePending } from '@/app/hooks';
import { useGetStateQuery, useSubscribeMessage, useUnsubscribeMessage } from '@/app/utils';
import cancelSVG from '@/assets/icons/cross-circle-icon.svg';
import playSVG from '@/assets/icons/play-icon.svg';
import playlistCrossedSVG from '@/assets/icons/playlist-crossed-icon.svg';
import { Modal } from '@/components';
import { Button } from '@/ui';
import { cx } from '@/utils';

import { SubscribeModalProps } from './SubscribeModal.interface';
import styles from './SubscribeModal.module.scss';

function SubscribeModal({ speakerId, type, onClose }: SubscribeModalProps) {
  const { subscribeMessage } = useSubscribeMessage();
  const { unsubscribeMessage } = useUnsubscribeMessage();
  const { pending } = usePending();
  const navigate = useNavigate();
  const { account } = useAccount();
  const { users, refetch } = useGetStateQuery();

  const handleCancelModal = () => {
    onClose();
  };

  const handleRedirectToAccount = () => {
    handleCancelModal();
    navigate(`/${ACCOUNT}`);
  };

  const handleSubscribe = (action: 'sub' | 'unsub') => {
    if (speakerId && speakerId.startsWith('0x')) {
      const sendMessage = action === 'sub' ? subscribeMessage : unsubscribeMessage;
      void sendMessage(
        { accountId: speakerId as HexString },
        {
          onError: () => handleCancelModal(),
          onSuccess: () => {
            void refetch();
            handleCancelModal();
          },
        },
      );
    }
  };

  return (
    <Modal heading={type === 'subscribe' ? 'Subscribe' : 'Unsubscribe'} onClose={onClose}>
      <div className={cx(styles.container)}>
        {account && users[account.decodedAddress] ? (
          <>
            <p className={cx(styles.description)}>
              Are you sure you want to {type} {type === 'subscribe' ? 'to' : 'from'} this streamer?
            </p>
            <div className={cx(styles.controls)}>
              {type === 'subscribe' && (
                <Button
                  variant="primary"
                  label="Subscribe"
                  isLoading={pending}
                  icon={playSVG}
                  onClick={() => handleSubscribe('sub')}
                />
              )}
              {type === 'unsubscribe' && (
                <Button
                  variant="primary"
                  label="Unsubscribe"
                  isLoading={pending}
                  icon={playlistCrossedSVG}
                  onClick={() => handleSubscribe('unsub')}
                  className={cx(styles['unsubscribe-button'])}
                />
              )}
              <Button variant="text" label="Cancel" icon={cancelSVG} onClick={handleCancelModal} />
            </div>
          </>
        ) : (
          <div className={cx(styles['create-account-container'])}>
            <span>Create an account first to subscribe</span>

            <Button variant="primary" label="Create an account" onClick={handleRedirectToAccount} />
          </div>
        )}
      </div>
    </Modal>
  );
}

export { SubscribeModal };
