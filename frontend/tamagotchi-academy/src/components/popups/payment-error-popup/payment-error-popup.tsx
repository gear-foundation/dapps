import { Button, Modal } from '@gear-js/ui'
import { useApp } from '@/app/context'
import { useGetFTBalance } from '@/app/hooks/use-ft-balance'
import { SpriteIcon } from '@/components/ui/sprite-icon'

type Props = {
  close: () => void
}

export const PaymentErrorPopup = ({ close }: Props) => {
  const { isPending } = useApp()
  const { handler } = useGetFTBalance()

  const onClick = () => {
    handler(onClose)
  }

  const onClose = () => {
    close()
  }

  return (
    <Modal heading="Payment error" close={close}>
      <div className="space-y-6">
        <p>
          There are not enough funds on your account, please replenish the
          balance using the "Get Token Balance" button
        </p>
        <Button
          className="gap-2 w-full"
          color="primary"
          text="Get Token Balance"
          icon={() => <SpriteIcon name="money" className="w-5 h-5" />}
          onClick={onClick}
          disabled={isPending}
        />
      </div>
    </Modal>
  )
}
