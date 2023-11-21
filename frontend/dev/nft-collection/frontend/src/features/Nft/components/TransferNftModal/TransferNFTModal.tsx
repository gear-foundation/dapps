import { useMemo } from 'react';
import { useParams } from 'react-router-dom';
import { useSendMessage } from '@gear-js/react-hooks';
import { useForm } from '@mantine/form';
import { HexString } from '@polkadot/util/types';
import { getProgramMetadata } from '@gear-js/api';
import { Modal } from 'components';
import { isHex } from '@polkadot/util';
import { useNFTs } from '@/features/Nft/hooks';
import styles from './TransferNFTModal.module.scss';

const initialValues = { address: '' };
const validate = { address: (value: string) => (isHex(value) ? null : 'Address should be hex') };

type Params = {
  programId: HexString;
  id: string;
};

type Props = {
  onClose: () => void;
};

export function TransferNFTModal({ onClose }: Props) {
  const { NFTContracts } = useNFTs();
  const { programId, id } = useParams() as Params;
  const contract = NFTContracts.find(([address]: any) => address === programId);
  const metaRaw = contract?.[1];
  const metaHex = metaRaw ? (`0x${metaRaw}` as HexString) : undefined;
  const metadata = useMemo(() => (metaHex ? getProgramMetadata(metaHex) : undefined), [metaHex]);

  const { getInputProps, onSubmit, errors } = useForm({ initialValues, validate });
  const error = errors.address;
  const sendMessage = useSendMessage(programId, metadata);

  const handleTransfer = onSubmit((values) => {
    const payload = { Transfer: { to: values.address, nft_id: id } };

    sendMessage(payload, { onSuccess: onClose });
  });

  return (
    <Modal heading="Transfer NFT" onClose={onClose}>
      <form className={styles.form} onSubmit={handleTransfer}>
        <div>
          <input
            placeholder="0x01"
            /* eslint-disable-next-line react/jsx-props-no-spreading */
            {...getInputProps('address')}
            className={styles.input}
          />
          {error && <p className={styles.error}>{error}</p>}
        </div>

        <button type="submit" className={styles.button}>
          Transfer
        </button>
      </form>
    </Modal>
  );
}
