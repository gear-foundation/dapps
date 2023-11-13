import Identicon from '@polkadot/react-identicon';
import { Modal, Button } from '@gear-js/ui';

import styles from './TransactionModal.module.scss';
import { TransactionModalProps } from './TransactionModal.interface';

function TransactionModal({ fee, name, addressTo, addressFrom, onClose, onAbort, onConfirm }: TransactionModalProps) {
  const handleClose = () => {
    if (onAbort) {
      onAbort();
    }

    onClose?.();
  };

  const handleConfirm = () => {
    onConfirm();
    onClose?.();
  };

  return (
    <Modal heading="Transaction details" size="large" close={handleClose}>
      <h4 className={styles.contentHeading}>
        Sending transaction <span className={styles.headingName}>{name}</span>
      </h4>
      <p className={styles.contentFeeInfo}>Fees of {fee} will be applied to the submission</p>
      <div className={styles.contentTransactionInfo}>
        <Identicon value={addressFrom} size={28} theme="polkadot" className={styles.infoIcon} />
        <p className={styles.infoParticipant}>
          <span className={styles.participantSide}>From:</span>
          <span className={styles.participantValue}>{addressFrom}</span>
        </p>
        {addressTo && (
          <p className={styles.infoParticipant}>
            <span className={styles.participantSide}>To:</span>
            <span className={styles.participantValue}>{addressTo}</span>
          </p>
        )}
      </div>
      <div className={styles.contentButtons}>
        <Button text="Submit" className={styles.actionButton} onClick={handleConfirm} />
        <Button text="Cancel" color="light" className={styles.actionButton} onClick={handleClose} />
      </div>
    </Modal>
  );
}

export { TransactionModal };
