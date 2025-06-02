import { Button, Modal } from '@gear-js/ui';

import styles from './PurchaseSubscriptionApproveModal.module.scss';

type Props = { disabledSubmitButton: boolean; amount: string; close: () => void; onSubmit: () => void };

function PurchaseSubscriptionApproveModal({ disabledSubmitButton, amount, close, onSubmit }: Props) {
  return (
    <Modal heading="Purchase subscription" close={close}>
      <div className={styles.container}>
        You're going to transfer {amount} Tokens
        <Button text="Approve" onClick={onSubmit} disabled={disabledSubmitButton} />
      </div>
    </Modal>
  );
}

export { PurchaseSubscriptionApproveModal };
