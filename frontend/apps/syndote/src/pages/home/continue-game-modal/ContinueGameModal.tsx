import { Modal } from 'components/layout/modal';
import { Button } from '@gear-js/vara-ui';
import styles from './ContinueGameModal.module.scss';
import { GameDetails } from 'components/layout/game-details';
import { ReactComponent as VaraSVG } from 'assets/images/icons/vara-coin.svg';
import { ReactComponent as TVaraSVG } from 'assets/images/icons/tvara-coin.svg';
import { useAccountDeriveBalancesAll, useApi, useBalanceFormat } from '@gear-js/react-hooks';

type Props = {
  onReserve: () => void;
  onClose: () => void;
};

function ContinueGameModal({ onReserve, onClose }: Props) {
  const { isApiReady } = useApi();
  const { getFormattedBalance } = useBalanceFormat();
  const balances = useAccountDeriveBalancesAll();
  const balance =
    isApiReady && balances?.freeBalance ? getFormattedBalance(balances.freeBalance.toString()) : undefined;

  const VaraSvg = balance?.unit?.toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />;
  const items = [
    {
      name: 'Required amount of gas  required for the game',
      value: <>{VaraSvg} 18 VARA</>,
      key: '1',
    },
  ];

  return (
    <Modal
      heading="The contract requires more gas to continue the game"
      className={{ header: styles.modalHeader }}
      onClose={onClose}>
      <div className={styles.container}>
        <p className={styles.text}>
          Please reserve a new gas amount to continue the game. Any unused gas will be refunded upon completion of the
          game. If you don't reserve the required amount of gas, you won't be able to continue the game.
        </p>
        <GameDetails items={items} className={{ item: styles.gameDetailsItem }} />
        <Button text="Reserve gas" className={styles.button} onClick={onReserve} />
      </div>
    </Modal>
  );
}

export { ContinueGameModal };
