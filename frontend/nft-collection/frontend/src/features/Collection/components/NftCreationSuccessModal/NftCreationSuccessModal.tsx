import { useNavigate } from 'react-router-dom';
import { Modal } from 'components';
import { useAlert } from '@gear-js/react-hooks';
import { cx, copyToClipboard } from '@/utils';
import { ReactComponent as CopyToClipboardSVG } from '../../assets/images/copy-icon.svg';
import styles from './NftCreationSuccessModal.module.scss';
import { Button } from '@/ui';
import { NftCreationSuccessModalProps } from './NftCreationSuccessModal.interface';
import { COLLECTION } from '@/routes';

function NftCreationSuccessModal({ collectionId, onClose }: NftCreationSuccessModalProps) {
  const navigate = useNavigate();
  const alert = useAlert();

  const handleNavigateToCollection = () => {
    navigate(`${COLLECTION}/${collectionId}`);
  };

  const handleCopyLink = () => {
    copyToClipboard(`${window.location.origin}${COLLECTION}/${collectionId}`).then(() => alert.success('copied'));
  };

  return (
    <Modal heading="NftCreationSuccessModal" onClose={onClose} className={cx(styles.modal)}>
      <div className={cx(styles.content)}>
        <span className={cx(styles['content-heading'])}>
          Now you can mint your first NFT or share the collection link with your friends.
        </span>

        <div className={cx(styles['collection-link-container'])}>
          <span className={cx(styles['collection-link-text'])}>
            {window.location.origin}
            {COLLECTION}/{collectionId}
          </span>
          <button className={cx(styles['copy-link-wrapper'])} onClick={handleCopyLink}>
            <CopyToClipboardSVG />
            <span className={cx(styles['copy-link-text'])}>Copy link</span>
          </button>
        </div>
        <div className={cx(styles.buttons)}>
          <Button
            variant="primary"
            className={cx(styles.button)}
            label="Open Collection"
            onClick={handleNavigateToCollection}
          />
          <Button
            variant="primary"
            className={cx(styles.button, styles['button-grey'])}
            label="Continue"
            onClick={() => onClose?.()}
          />
        </div>
      </div>
    </Modal>
  );
}

export { NftCreationSuccessModal };
