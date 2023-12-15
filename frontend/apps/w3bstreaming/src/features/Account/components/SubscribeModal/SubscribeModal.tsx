import { useAtom } from 'jotai';
import { useNavigate } from 'react-router';
import { Loader, Modal } from 'components';
import { useAccount, useAlert, withoutCommas } from '@gear-js/react-hooks';
import { cx, logger } from '@/utils';
import playSVG from '@/assets/icons/play-icon.svg';
import cancelSVG from '@/assets/icons/cross-circle-icon.svg';
import playlistCrossedSVG from '@/assets/icons/playlist-crossed-icon.svg';
import { Button } from '@/ui';
import { SubscribeModalProps } from './SubscribeModal.interface';
import { useSubscribeToStreamMessage } from '../../hooks';
import styles from './SubscribeModal.module.scss';
import { useGetStreamMetadata } from '@/features/CreateStream/hooks';
import { useCheckBalance, useHandleCalculateGas, useProgramState } from '@/hooks';
import { ADDRESS } from '@/consts';
import { IS_SUBSCRIBING_ATOM } from '../../atoms';
import { ACCOUNT } from '@/App.routes';

function SubscribeModal({ speakerId, type, onClose }: SubscribeModalProps) {
  const { meta, isMeta } = useGetStreamMetadata();
  const [isLoading, setIsLoading] = useAtom(IS_SUBSCRIBING_ATOM);
  const sendSubscribeMessage = useSubscribeToStreamMessage();
  const calculateGas = useHandleCalculateGas(ADDRESS.CONTRACT, meta);
  const { checkBalance } = useCheckBalance();
  const alert = useAlert();
  const navigate = useNavigate();
  const { account } = useAccount();
  const {
    state: { users },
  } = useProgramState();
  const { updateUsers } = useProgramState();

  const handleCancelModal = () => {
    onClose();
  };

  const handleRedirectToAccount = () => {
    handleCancelModal();
    navigate(`/${ACCOUNT}`);
  };

  const handleSubscribe = (action: 'sub' | 'unsub') => {
    if (speakerId) {
      setIsLoading(true);

      const payload =
        action === 'sub'
          ? {
              Subscribe: {
                account_id: speakerId,
              },
            }
          : {
              Unsubscribe: {
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
                  setIsLoading(false);
                  handleCancelModal();
                },
                onSuccess: (messageId) => {
                  logger(`sucess on ID: ${messageId}`);
                  updateUsers();
                  setIsLoading(false);
                  handleCancelModal();
                },
                onInBlock: (messageId) => {
                  logger('messageInBlock');
                  logger(`messageID: ${messageId}`);
                },
              }),
            () => {
              logger(`Errror check balance`);
              setIsLoading(false);
              handleCancelModal();
            },
          );
        })
        .catch((error) => {
          logger(error);
          alert.error('Gas calculation error');
          setIsLoading(false);
          handleCancelModal();
        });
    }
  };

  return (
    <Modal heading={type === 'subscribe' ? 'Subscribe' : 'Unsubscribe'} onClose={onClose}>
      {isMeta ? (
        <div className={cx(styles.container)}>
          {users?.[account?.decodedAddress || ''] ? (
            <>
              <p className={cx(styles.description)}>
                Are you sure you want to {type} {type === 'subscribe' ? 'to' : 'from'} this streamer?
              </p>
              <div className={cx(styles.controls)}>
                {type === 'subscribe' && (
                  <Button
                    variant="primary"
                    label="Subscribe"
                    isLoading={isLoading}
                    icon={playSVG}
                    onClick={() => handleSubscribe('sub')}
                  />
                )}
                {type === 'unsubscribe' && (
                  <Button
                    variant="primary"
                    label="Unsubscribe"
                    isLoading={isLoading}
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
      ) : (
        <Loader />
      )}
    </Modal>
  );
}

export { SubscribeModal };
