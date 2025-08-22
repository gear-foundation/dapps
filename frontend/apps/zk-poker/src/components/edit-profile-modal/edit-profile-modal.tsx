import { Button, Input, Modal } from '@gear-js/vara-ui';
import { useState } from 'react';

import styles from './edit-profile-modal.module.scss';

type Props = {
  userName: string;
  onClose: () => void;
  onSave: (name: string) => void;
};

const EditProfileModal = ({ userName, onClose, onSave }: Props) => {
  const [name, setName] = useState(userName);

  const handleNameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setName(e.target.value);
  };

  const handleSave = () => {
    onSave(name);
  };

  return (
    <Modal heading="Update name" close={onClose} className={styles.modal}>
      <Input placeholder="Enter your name" value={name} onChange={handleNameChange} maxLength={20} />

      <div className={styles.actions}>
        <Button color="grey" size="small" onClick={onClose}>
          Cancel
        </Button>
        <Button color="primary" size="small" onClick={handleSave}>
          Save
        </Button>
      </div>
    </Modal>
  );
};

export { EditProfileModal };
