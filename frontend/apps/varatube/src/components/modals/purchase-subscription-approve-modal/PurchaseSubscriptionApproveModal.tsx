import { Button, Modal } from '@gear-js/ui';
import styles from './PurchaseSubscriptionApproveModal.module.scss';

type Props = { amount: string; close: () => void; onSubmit: () => void };

function PurchaseSubscriptionApproveModal({ amount, close, onSubmit }: Props) {
  return (
    <Modal heading="Purchase subscription" close={close}>
      <div className={styles.container}>
        You're going to transfer {amount} Vara
        <Button text="Approve" onClick={onSubmit} />
      </div>
    </Modal>
  );
}

export { PurchaseSubscriptionApproveModal };
