import { Button, Modal } from '@gear-js/ui';

import { useApp } from '@/app/context';
import { useTamagotchiMessage } from '@/app/hooks/use-tamagotchi';

export const RevokeApprovalPopup = ({ close }: { close: () => void }) => {
  const { isPending } = useApp();
  const sendHandler = useTamagotchiMessage();
  const onSuccess = () => close();
  const handler = () => sendHandler({ RevokeApproval: null }, { onSuccess });

  return (
    <Modal heading="Revoke approval" close={close}>
      <div className="flex gap-6">
        <Button text="Dismiss" color="secondary" onClick={close} disabled={isPending} />
        <Button text="Ok" color="primary" onClick={handler} disabled={isPending} />
      </div>
    </Modal>
  );
};
