import cancelSVG from '@/assets/icons/cross-circle-icon.svg';
import { Modal } from '@/components';
import { Button } from '@/ui';
import { cx } from '@/utils';

import { WalletModalProps } from './CreateStreamRestrictModal.interface';
import styles from './CreateStreamRestrictModal.module.scss';

function CreateStreamRestrictModal({ onClose }: WalletModalProps) {
  const handleCancelModal = () => {
    onClose();
  };

  return (
    <Modal heading="Stream creation error" onClose={onClose}>
      <div className={cx(styles.container)}>
        <p className={cx(styles.description)}>
          In order to schedule a stream, you need to create an account on our streaming service.
        </p>
        <div className={cx(styles.controls)}>
          <Button variant="text" label="Cancel" icon={cancelSVG} onClick={handleCancelModal} />
        </div>
      </div>
    </Modal>
  );
}

export { CreateStreamRestrictModal };
