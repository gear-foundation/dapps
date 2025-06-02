import { useBalanceFormat } from '@gear-js/react-hooks';
import { Button, Modal, ModalProps } from '@gear-js/vara-ui';

import { Text } from '@/components';
import { VaraIcon } from '@/components/layout';

import styles from './tournament-result-modal.module.scss';

type Props = Pick<ModalProps, 'close'> & {
  bid: number;
  participantsCount: number;
  winnerNames: [string, string | undefined];
  startOverButton: { isLoading: boolean; onClick: () => Promise<void> };
};

function TournamentResultModal({ bid, participantsCount, winnerNames, startOverButton, close }: Props) {
  const { getFormattedBalanceValue } = useBalanceFormat();
  const prizeValue = getFormattedBalanceValue(bid).toNumber() * participantsCount;

  const [firstWinnerName, secondWinnerName] = winnerNames;
  const isDraw = Boolean(secondWinnerName);

  return (
    <Modal heading="" close={close} maxWidth="large">
      <div className={styles.content}>
        <h3 className={styles.heading}>
          {isDraw ? `${firstWinnerName} and ${secondWinnerName} ended in a draw!` : `${firstWinnerName} wins!`}
        </h3>

        <div className={styles.text}>
          <p>The final battle is over. Thank you for playing!</p>
          <p>Try your skills in a new battle or go back to check the stats of previous ones.</p>
        </div>

        <div className={styles.prize}>
          <Text size="sm">Winner prize:</Text>
          <VaraIcon />

          <Text size="sm" weight="semibold">
            {isDraw ? prizeValue / 2 : prizeValue} VARA
          </Text>
        </div>
      </div>

      <div className={styles.buttons}>
        <Button text="Back" color="grey" onClick={close} />
        <Button text="Start Over" {...startOverButton} />
      </div>
    </Modal>
  );
}

export { TournamentResultModal };
