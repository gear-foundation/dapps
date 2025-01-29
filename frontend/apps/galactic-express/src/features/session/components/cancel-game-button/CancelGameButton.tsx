import clsx from 'clsx';
import { ReactComponent as CrossIconSVG } from '@/assets/images/icons/cross-icon.svg';
import { useAtom, useSetAtom } from 'jotai';
import { Button } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';
import { Participant } from '@/features/session/types';
import { IS_LOADING, REGISTRATION_STATUS } from '@/atoms';
import { useCancelGameMessage, useCancelRegisterMessage } from '@/app/utils';
import styles from './CancelGameButton.module.scss';

type Props = {
  isAdmin: boolean;
  participants: Participant[];
};

function CancelGameButton({ isAdmin, participants }: Props) {
  const setRegistrationStatus = useSetAtom(REGISTRATION_STATUS);
  const [isLoading, setIsLoading] = useAtom(IS_LOADING);
  const { account } = useAccount();

  const { cancelGameMessage } = useCancelGameMessage();
  const { cancelRegisterMessage } = useCancelRegisterMessage();

  const isRegistered = account?.decodedAddress
    ? participants.map((participant) => participant[0]).includes(account.decodedAddress)
    : false;

  const onError = () => {
    setIsLoading(false);
  };

  const onSuccess = () => {
    setIsLoading(false);
    setRegistrationStatus('registration');
  };

  const handleClick = () => {
    setIsLoading(true);
    if (isAdmin) {
      cancelGameMessage({ onError, onSuccess });
    }
    if (!isAdmin && isRegistered) {
      cancelRegisterMessage({ onError, onSuccess });
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
