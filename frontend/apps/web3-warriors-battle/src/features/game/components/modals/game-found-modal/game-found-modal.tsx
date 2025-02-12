import { useApi } from '@gear-js/react-hooks';
import { Button, Input } from '@gear-js/vara-ui';
import { isNotEmpty, useForm } from '@mantine/form';

import { MAX_PLAYERS_COUNT } from '@/app/consts';
import { UserIcon } from '@/assets/images';
import TVaraSVG from '@/assets/images/icons/tvara-coin.svg?react';
import VaraSVG from '@/assets/images/icons/vara-coin.svg?react';
import { GameDetails } from '@/components/layout';
import { Modal } from '@/components/ui/modal';
import { usePending } from '@/features/game/hooks';

import styles from './game-found-modal.module.scss';

type Props = {
  entryFee: number | string;
  participantsCount: number;
  onSubmit: (values: JoinModalFormValues) => Promise<void>;
  onClose: () => void;
};

export type JoinModalFormValues = {
  name: string;
};

function GameFoundModal({ entryFee, participantsCount, onSubmit, onClose }: Props) {
  const { api } = useApi();
  const { pending } = usePending();

  const VaraSvg = api?.registry.chainTokens[0].toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />;

  const items = [
    {
      name: 'Entry fee',
      value: (
        <>
          {VaraSvg} {entryFee} VARA
        </>
      ),
      key: '1',
    },
    {
      name: 'Players already joined the game',
      value: (
        <>
          <UserIcon /> {participantsCount} / {MAX_PLAYERS_COUNT}
        </>
      ),
      key: '2',
    },
  ];

  const joinForm = useForm({
    initialValues: {
      name: '',
    },
    validate: {
      name: isNotEmpty(`Name shouldn't be empty`),
    },
  });

  const { errors: joinErrors, getInputProps: getJoinInputProps, onSubmit: onJoinSubmit } = joinForm;

  const handleJoinSession = async (values: JoinModalFormValues) => {
    await onSubmit(values);
  };

  return (
    <Modal
      title="The game has been found"
      description="To proceed, review the parameters of the gaming session and click the “Join” button. If applicable, you will
          need to pay the entry fee and required amount of gas immediately after clicking the “Join” button. After the
          end of the game, any unused gas will be refunded."
      className={styles.modal}
      onClose={onClose}>
      <GameDetails items={items} />
      <form className={styles.form} onSubmit={onJoinSubmit(handleJoinSession)}>
        <div className={styles.input}>
          <Input label="Enter your name:" placeholder="Username" maxLength={20} {...getJoinInputProps('name')} />
          <span className={styles.fieldError}>{joinErrors.name}</span>
        </div>
        <div className={styles.buttons}>
          <Button text="Cancel" color="grey" disabled={pending} className={styles.button} onClick={onClose} />
          <Button type="submit" text="Join" disabled={pending} className={styles.button} />
        </div>
      </form>
    </Modal>
  );
}

export { GameFoundModal };
