import { Logo } from './logo';
import { Header as CommonHeader, MenuHandler } from '@dapps-frontend/ui';
import { Button } from '@gear-js/vara-ui';
import clsx from 'clsx';
import { useAccount } from '@gear-js/react-hooks';

import styles from './header.module.scss';
import { Icon } from 'components/ui/icon';
import { useApp, useGame } from 'app/context';
import { useState } from 'react';
import { Modal } from 'components/ui/modal';
import { useGameMessage } from 'app/hooks/use-game';

export function Header() {
  const { account } = useAccount();
  const { setIsPending, isPending } = useApp();
  const { isAdmin } = useGame()
  const [isOpenModal, setIsOpenModal] = useState(false)
  const handleMessage = useGameMessage();

  const onSuccess = () => {
    setIsOpenModal(false)
    setIsPending(false);
  };
  const onError = () => {
    setIsOpenModal(false)
    setIsPending(false);
  };

  const onCloseGame = () => {
    setIsPending((prev) => !prev);
    handleMessage({
      payload: { CancelGame: null },
      onSuccess,
      onError,
    });
    setIsOpenModal(false)
  }

  return (
    <CommonHeader
      logo={
        <Logo className={clsx(styles.header__logo, !account && styles['header__logo--center'])} label="Tic-Tac-Toe" />
      }
      className={{ header: styles.header, content: styles.header__container }}
      menu={
        <div className="flex items-center">
          {isOpenModal &&
            <Modal heading='Sure you want to end the game?' onClose={() => setIsOpenModal(false)}>
              This action cannot be undone. The game will be concluded, and all players will exit the gaming room. Any entry fees will be refunded to all players.

              <div className="flex w-full gap-3 mt-5">
                <Button
                  text='Cancel game'
                  color='grey'
                  onClick={onCloseGame}
                  disabled={isPending}
                />
                <Button
                  text='Continue the game'
                  color='primary'
                  onClick={() => setIsOpenModal(false)}
                  disabled={isPending}
                />
              </div>
            </Modal>
          }
          {isAdmin &&
            <Button
              text='Cancel game'
              className="!bg-[#F7ECED] !text-red-500 mr-5"
              icon={() => <Icon name="close" width={18} height={18} />}
              onClick={() => setIsOpenModal(true)}
            />}
          <MenuHandler />
        </div>
      }
    />
  );
}
