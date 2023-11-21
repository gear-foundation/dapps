import { useCallback, useEffect, useState } from 'react';
import { useNavigate } from 'react-router';
import { useForm } from '@mantine/form';
import { useAtomValue } from 'jotai';
import { UserMessageSent } from '@gear-js/api';
import { UnsubscribePromise } from '@polkadot/api/types';
import { useApi } from '@gear-js/react-hooks';
import { Vec, u8 } from '@polkadot/types';
import { Button, DropzoneUploader } from '@/ui';
import { ContractFormValues, CreateCollectionProps, DecodedReply } from './CreateCollection.interfaces';
import styles from './CreateCollection.module.scss';
import { cx } from '@/utils';
import icCloud from '../../assets/images/ic-cloud-upload-24.svg';
import 'swiper/css';
import 'swiper/css/navigation';
import { ACCOUNT_ATOM } from '@/atoms';
import { useFactoryMessage } from '../../hooks';
import { NftCreationSuccessModal } from '../NftCreationSuccessModal';
import { ADDRESS } from '@/consts';
import { TESTNET_USERNAME_ATOM } from '@/features/Auth/atoms';

const collectionName = 'NFT collection on Vara Incentivized Testnet';
const collectionDescription = (name: string) =>
  `Welcome to ${name}'s enchanting NFT collection crafted on Vara Incentivized Testnet. Embark on a mesmerizing journey through the boundless expanse of digital realms, where imagination meets technology, and creativity knows no bounds.`;

function CreateCollection() {
  const navigate = useNavigate();
  const [isSuccessModalOpen, setIsSuccessModalOpen] = useState<boolean>(false);
  const [newCollectionId, setNewCollectionId] = useState<string | null>('');
  const testnetUserName = useAtomValue(TESTNET_USERNAME_ATOM);
  const { api } = useApi();
  const account = useAtomValue(ACCOUNT_ATOM);
  const { meta: factoryMeta, message: factoryMessage } = useFactoryMessage();

  const form = useForm<ContractFormValues>({
    initialValues: {
      name: `${testnetUserName} ${collectionName}`,
      description: collectionDescription(account?.meta.name || ''),
      media: [],
    },
    validate: {},
  });

  const { onSubmit, setFieldValue, reset } = form;

  const handleDropFile = useCallback(
    (previews: string[]) => {
      setFieldValue('media', previews);
    },
    [setFieldValue],
  );

  const handleCreateCollection = ({ name, description, media }: ContractFormValues) => {
    const payload = {
      CreateCollection: {
        name,
        description,
        media,
      },
    };

    factoryMessage(payload);
  };

  const handleContinue = () => {
    reset();
    setIsSuccessModalOpen(false);
    setNewCollectionId(null);
  };

  useEffect(() => {
    if (account) {
      setFieldValue('name', `${testnetUserName} ${collectionName}`);
      setFieldValue('description', collectionDescription(testnetUserName || ''));
    }
  }, [account, setFieldValue, testnetUserName]);

  const getDecodedPayload = (payload: Vec<u8>) => {
    if (factoryMeta?.types.handle.output) {
      return factoryMeta.createType(factoryMeta.types.handle.output, payload).toHuman();
    }
  };

  const getDecodedReply = (payload: Vec<u8>): DecodedReply => {
    const decodedPayload = getDecodedPayload(payload);

    return decodedPayload as DecodedReply;
  };

  const handleEvents = ({ data }: UserMessageSent) => {
    const { message } = data;
    const { destination, source, payload } = message;
    const isOwner = destination.toHex() === account?.decodedAddress;
    const isEscrowProgram = source.toHex() === ADDRESS.FACTORY;

    if (isOwner && isEscrowProgram) {
      const reply = getDecodedReply(payload);

      console.log(reply);

      if (reply && reply.CollectionCreated) {
        setNewCollectionId(reply.CollectionCreated.collectionAddress);
        setIsSuccessModalOpen(true);
        reset();
      }
    }
  };

  useEffect(() => {
    let unsub: UnsubscribePromise | undefined;

    if (api && account?.decodedAddress && factoryMeta) {
      unsub = api.gearEvents.subscribeToGearEvent('UserMessageSent', handleEvents);
    }

    return () => {
      if (unsub) unsub.then((unsubCallback) => unsubCallback());
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, account?.decodedAddress, factoryMeta]);

  const handleCancel = () => {
    navigate(-1);
  };

  return (
    <>
      <form onSubmit={onSubmit(handleCreateCollection)} className={cx(styles.container)}>
        <h1 className={cx(styles.title)}>New Collection</h1>
        <div className={cx(styles.block)}>
          <div className={cx(styles.content)}>
            <span className={cx(styles['block-title'])}>Name</span>
            <span className={cx(styles['block-name'])}>
              {testnetUserName} {collectionName}
            </span>
          </div>
          <div className={cx(styles.content)}>
            <span className={cx(styles['block-title'])}>Description</span>
            <span className={cx(styles['block-description'])}>{collectionDescription(testnetUserName || '')}</span>
          </div>
          <div className={cx(styles.uploader)}>
            <div className={cx(styles.content)}>
              <span className={cx(styles['block-name'])}>Upload images (10 of 10)</span>
              <span className={cx(styles['block-title'])}>You can upload .jpg, .jpeg, .png, .gif files here</span>
            </div>
            <DropzoneUploader
              multi
              onDropFile={handleDropFile}
              content={
                <div className={cx(styles['dropzone-content'])}>
                  <img src={icCloud} alt="ff" className={cx(styles['dropzone-content-image'])} />
                  <div className={cx(styles['dropzone-content-choose'])}>Choose File</div>
                  <span className={cx(styles['dropzone-content-text'])}>Or drag and drop your files here</span>
                </div>
              }
            />
          </div>
          <div className={cx(styles.buttons)}>
            <Button variant="primary" className={cx(styles.button)} label="Create collection" type="submit" />
            <Button
              variant="primary"
              className={cx(styles.button, styles['button-grey'])}
              label="Cancel"
              onClick={handleCancel}
            />
          </div>
        </div>
      </form>
      {isSuccessModalOpen && <NftCreationSuccessModal collectionId={newCollectionId} onClose={handleContinue} />}
    </>
  );
}

export { CreateCollection };
