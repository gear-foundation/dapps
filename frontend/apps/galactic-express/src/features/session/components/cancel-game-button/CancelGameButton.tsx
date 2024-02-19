import { ReactComponent as CrossIconSVG } from 'assets/images/icons/cross-icon.svg';
import { useAtom } from 'jotai';
import { Button } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';
import { useLaunchMessage } from 'features/session/hooks';
import { Participant } from 'features/session/types';
import { IS_LOADING } from 'atoms';
import styles from './CancelGameButton.module.scss';

type Props = {
  isAdmin: boolean;
  userAddress: string;
  participants: Participant[];
};

function CancelGameButton({ isAdmin, participants, userAddress }: Props) {
  const { meta: isMeta, message: sendMessage } = useLaunchMessage();
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

  return isRegistered || isAdmin ? (
    <div className={styles.buttonWrapper}>
      <Button
        text="Cancel"
        icon={CrossIconSVG}
        color="light"
        className={styles.button}
        onClick={handleClick}
        isLoading={isLoading}
      />
    </div>
  ) : null;
}

export { CancelGameButton };
