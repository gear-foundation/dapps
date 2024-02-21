import { Modal } from 'components/layout/modal';
import { Button } from '@gear-js/vara-ui';
import styles from './TextModal.module.scss';

type Props = {
  heading: string;
  text: string;
  onClose: () => void;
};

export type JoinModalFormValues = {
  name: string;
};

function TextModal({ heading, text, onClose }: Props) {
  return (
    <Modal heading={heading} className={{ header: styles.modalHeader }} onClose={onClose}>
      <div className={styles.container}>
        <p className={styles.text}>{text}</p>

        <Button text="OK" color="grey" className={styles.button} onClick={onClose} />
      </div>
    </Modal>
  );
}

export { TextModal };
