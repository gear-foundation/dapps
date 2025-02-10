import { useBalanceFormat, useAccount, withoutCommas } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';

import { GameDetails } from '@/components/layout/game-details';
import { Modal } from '@/components/layout/modal';
import { useQuitGame } from '@/hooks/useQuitGame';
import { PlayersByStrategyAddress } from '@/types';

import { VaraIcon } from '../vara-icon';

import styles from './GameFinishedModal.module.scss';

type Props = {
  winnerAddress: `0x${string}`;
  prizePool: string | undefined;
  isAdmin: boolean;
  players: PlayersByStrategyAddress;
  onClose: () => void;
};

function GameFinishedModal({ winnerAddress, isAdmin, players, prizePool = '0', onClose }: Props) {
  const { account } = useAccount();
  const { getFormattedBalanceValue } = useBalanceFormat();
  const { deleteGame, exitGame } = useQuitGame();

  const isWinner = account?.decodedAddress === players[winnerAddress].ownerId;
  const winnerName = players[winnerAddress].name;

  const items = [
    {
      name: `Winner's prize:`,
      value: (
        <>
          <VaraIcon /> {getFormattedBalanceValue(withoutCommas(prizePool) || 0).toFixed(2)} VARA
        </>
      ),
      key: '1',
    },
  ];

  return (
    <Modal
      heading={isWinner ? 'You won the game!' : 'The game is over!'}
      className={{ header: styles.modalHeader }}
      onClose={onClose}>
      <div className={styles.container}>
        <p className={styles.text}>
          {isWinner
            ? 'You are the winner! You take the entire prize pool.'
            : `Player ${winnerName} is the winner. He takes the entire prize pool.`}
        </p>
        <GameDetails items={items} className={{ item: styles.gameDetailsItem }} />
        <div className={styles.controls}>
          <Button text="Close" className={styles.button} onClick={onClose} color="grey" />
          <Button
            text={isAdmin ? 'Play again' : 'Quit'}
            className={styles.button}
            onClick={isAdmin ? deleteGame : exitGame}
          />
        </div>
      </div>
    </Modal>
  );
}

export { GameFinishedModal };
