import { ReactComponent as CrossIconSVG } from 'assets/images/icons/cross-icon.svg';
import { useAtom, useAtomValue, useSetAtom } from 'jotai';
import { Button } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';
import { useLaunchMessage } from 'features/session/hooks';
import { Participant } from 'features/session/types';
import { IS_LOADING, REGISTRATION_STATUS } from 'atoms';
import styles from './CancelGameButton.module.scss';
import clsx from 'clsx';

type Props = {
  isAdmin: boolean;
  participants: Participant[];
};

function CancelGameButton({ isAdmin, participants }: Props) {
  const { meta: isMeta, message: sendMessage } = useLaunchMessage();
  const setRegistrationStatus = useSetAtom(REGISTRATION_STATUS);
  const [isLoading, setIsLoading] = useAtom(IS_LOADING);
  const { account } = useAccount();

  const isRegistered = account?.decodedAddress
    ? participants.map((participant) => participant[0]).includes(account.decodedAddress)
    : false;

  const onError = () => {
    setIsLoading(false);
  };

  const onInBlock = () => {
    setIsLoading(false);
    setRegistrationStatus('registration');
  };

  const handleClick = () => {
    setIsLoading(true);
    if (isAdmin) {
      sendMessage({
        payload: {
          CancelGame: null,
        },
        onError,
        onInBlock,
      });
    }

    if (!isAdmin && isRegistered) {
      sendMessage({
        payload: {
          CancelRegistration: null,
        },
        onError,
        onInBlock,
      });
    }
  };

  return (
    <div className={clsx(isAdmin ? styles.buttonWrapperAdmin : styles.buttonWrapper)}>
      <Button
        text={isAdmin ? 'Cancel game' : 'Cancel'}
        icon={CrossIconSVG}
        color="light"
        className={styles.button}
        onClick={handleClick}
        isLoading={isLoading}
      />
    </div>
  );
}

export { CancelGameButton };
