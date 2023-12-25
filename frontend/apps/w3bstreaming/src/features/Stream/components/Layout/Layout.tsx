import { useNavigate } from 'react-router-dom';
import { useEffect, useState } from 'react';
import { useAccount } from '@gear-js/react-hooks';
import styles from './Layout.module.scss';
import { cx, socket } from '@/utils';
import { Button } from '@/ui';
import speakerPhoto from '@/assets/icons/no-avatar-user-img.png';
import editProfileSVG from '@/assets/icons/edit-profile-icon.svg';
import timerSVG from '@/assets/icons/timer-icon.svg';
import eyeSVG from '@/assets/icons/eye-icon.svg';
import { LayoutProps } from './Layout.interfaces';
import { SubscribeModal } from '@/features/Account/components/SubscribeModal';

function Layout({
  isBroadcaster,
  title,
  description,
  startTime,
  broadcasterInfo,
  broadcasterId,
  isUserSubscribed,
  streamId,
}: LayoutProps) {
  const [isSubscribeModalOpen, setIsSubscribeModalOpen] = useState<boolean>(false);
  const [connectionsCount, setConnectionsCount] = useState<number>(0);
  const [isStreamGoing, setIsStreamGoing] = useState<boolean>(false);
  const navigate = useNavigate();
  const { account } = useAccount();

  const handleRedirectToAccount = () => {
    navigate('/account');
  };

  const handleCloseSubscribeModal = () => {
    setIsSubscribeModalOpen(false);
  };

  const handleOpenSubscribeModal = () => {
    setIsSubscribeModalOpen(true);
  };

  useEffect(() => {
    if (account?.decodedAddress) {
      socket.emit('getWatchersCount', account?.decodedAddress, { streamId });
      socket.emit('getIsStreaming', account?.decodedAddress, { streamId });

      socket.on('watchersCount', (connections) => {
        setConnectionsCount(connections);
      });

      socket.on('isStreaming', (isStreaming) => {
        setIsStreamGoing(isStreaming);
      });
    }
  }, [streamId, account?.decodedAddress]);

  return (
    <div className={cx(styles.layout)}>
      <div className={cx(styles.left)}>
        <div className={cx(styles.title)}>{title}</div>
        <div className={cx(styles['card-top-speaker-container'])}>
          <div className={cx(styles['card-top-speaker'])}>
            <div className={cx(styles['card-top-speaker-photo-wrapper'])}>
              <img
                className={cx(
                  styles['card-top-speaker-photo'],
                  isStreamGoing ? styles['card-top-speaker-photo-on-air'] : '',
                )}
                src={broadcasterInfo?.imgLink || speakerPhoto}
                alt=""
              />
              {isStreamGoing && <div className={cx(styles['card-top-speaker-photo-caption-on-air'])}>on air</div>}
            </div>
            <div className={cx(styles['card-top-speaker-content'])}>
              <span className={cx(styles['card-top-speaker-name'])}>
                {broadcasterInfo?.name} {broadcasterInfo?.surname}
              </span>
              <span className={cx(styles['card-top-speaker-descr'])}>Speaker</span>
            </div>
          </div>
        </div>
        <div className={cx(styles['stream-info'])}>Stream Info</div>
        <div className={cx(styles['stream-description'])}>{description}</div>
      </div>
      <div className={cx(styles.right)}>
        <div className={cx(styles['views-and-time'])}>
          {isStreamGoing && (
            <div className={cx(styles.views)}>
              <img src={eyeSVG} alt="views" />
              <span>{connectionsCount}</span>
            </div>
          )}
          {/* <div className={cx(styles.time)}>
            <img src={timerSVG} alt="time" />
            <span>
              {startTime.getHours()}:{startTime.getMinutes().toString().padStart(2, '0')}
            </span>
          </div> */}
        </div>
        {isBroadcaster ? (
          <Button variant="outline" label="Edit Profile" icon={editProfileSVG} onClick={handleRedirectToAccount} />
        ) : (
          <>
            {isUserSubscribed ? (
              <Button
                variant="primary"
                label="Unsubscribe"
                onClick={handleOpenSubscribeModal}
                className={cx(styles['unsubscribe-button'])}
              />
            ) : (
              <Button variant="primary" label="Subscribe" onClick={handleOpenSubscribeModal} />
            )}
          </>
        )}
      </div>
      {isSubscribeModalOpen && (
        <SubscribeModal
          type={isUserSubscribed ? 'unsubscribe' : 'subscribe'}
          onClose={handleCloseSubscribeModal}
          speakerId={broadcasterId}
        />
      )}
    </div>
  );
}

export { Layout };
