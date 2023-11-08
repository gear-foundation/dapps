import { useMemo } from 'react'
import { useParams } from 'react-router-dom'
import { useSendMessage } from '@gear-js/react-hooks'
import { useForm } from '@mantine/form'
import { useNFTs } from 'features/nfts/hooks'
import { HexString } from '@polkadot/util/types'
import { ProgramMetadata } from '@gear-js/api'

import { Modal } from 'components'
import { isHex } from '@polkadot/util'
import styles from './TransferNFTModal.module.scss'

const initialValues = { address: '' }
const validate = {
  address: (value: string) => (isHex(value) ? null : 'Address should be hex'),
}

type Params = {
  id: string
}

type Props = {
  onClose: () => void
}

export function TransferNFTModal({ onClose }: Props) {
  // const { NFTContracts } = useNFTs();
  const { id } = useParams() as Params
  // const contract = NFTContracts.find(([address]) => address === programId);
  // const metaRaw = contract?.[1];
  // const metadata = useMemo(() => (metaRaw ? ProgramMetadata.from(`0x${metaRaw}`) : undefined), [metaRaw]);

  const { getInputProps, onSubmit, errors } = useForm({
    initialValues,
    validate,
  })
  const error = errors.address
  // const sendMessage = useSendMessage(programId, metadata);

  const handleTransfer = onSubmit((values) => {
    const payload = { Transfer: { to: values.address, nft_id: id } }

    // sendMessage(payload, { onSuccess: onClose });
  })

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
  )
}
