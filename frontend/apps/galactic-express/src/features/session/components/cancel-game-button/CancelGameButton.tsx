import { useAccount } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';
import { useAtom, useSetAtom } from 'jotai';

import { useCancelGameMessage, useCancelRegisterMessage } from '@/app/utils';
import CrossIconSVG from '@/assets/images/icons/cross-icon.svg?react';
import { IS_LOADING, REGISTRATION_STATUS } from '@/atoms';
import { Participant } from '@/features/session/types';

import styles from './CancelGameButton.module.scss';

type Props = {
  isAdmin: boolean;
  participants: Participant[];
};

function CancelGameButton({ isAdmin, participants }: Props) {
  const setRegistrationStatus = useSetAtom(REGISTRATION_STATUS);
  const [isLoading] = useAtom(IS_LOADING);
  const { account } = useAccount();

  const { cancelGameMessage } = useCancelGameMessage();
  const { cancelRegisterMessage } = useCancelRegisterMessage();

  const isRegistered = account?.decodedAddress
    ? participants.map((participant) => participant[0]).includes(account.decodedAddress)
    : false;

  const onSuccess = () => {
    setRegistrationStatus('registration');
  };

  const handleClick = () => {
    if (isAdmin) {
      void cancelGameMessage({ onSuccess });
    }
    if (!isAdmin && isRegistered) {
      void cancelRegisterMessage({ onSuccess });
    }
  };

  return (
    <div className={clsx(isAdmin ? styles.buttonWrapperAdmin : styles.buttonWrapper)}>
      <Button
        text={isAdmin ? 'Cancel game' : 'Cancel'}
        icon={CrossIconSVG}
        color="border"
        className={styles.button}
        onClick={handleClick}
        isLoading={isLoading}
      />
    </div>
  );
}

export { CancelGameButton };
