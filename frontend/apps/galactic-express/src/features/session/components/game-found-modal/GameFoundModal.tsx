import { useApi } from '@gear-js/react-hooks';
import { Button } from '@gear-js/vara-ui';
import { isNotEmpty, useForm } from '@mantine/form';

import TVaraSVG from '@/assets/images/icons/tvara-coin.svg?react';
import VaraSVG from '@/assets/images/icons/vara-coin.svg?react';
import { TextField } from '@/components/layout/TextField';
import { Modal } from '@/components/layout/modal';
import { cx } from '@/utils';

import UserSVG from '../../assets/ic-user-small-24.svg?react';

import styles from './GameFoundModal.module.scss';

type Props = {
  entryFee: number | string;
  players: number;
  gasAmount: number | string;
  onSubmit: (values: JoinModalFormValues) => void;
  onClose: () => void;
};

export type JoinModalFormValues = {
  name: string;
};

function GameFoundModal({ entryFee, players, gasAmount, onSubmit, onClose }: Props) {
  const { api } = useApi();
  const VaraSvg = api?.registry.chainTokens[0].toLowerCase() === 'vara' ? <VaraSVG /> : <TVaraSVG />;

  const items = [
    {
      name: 'Entry fee',
      value: (
        <>
          {VaraSvg} {entryFee} VARA
        </>
      ),
    },
    {
      name: 'Players already joined the game',
      value: (
        <>
          <UserSVG /> {players} / 4
        </>
      ),
    },
    {
      name: 'Required gas amount ',
      value: (
        <>
          {VaraSvg} {gasAmount} VARA
        </>
      ),
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

  const handleJoinSession = (values: JoinModalFormValues) => {
    onSubmit(values);
  };

  return (
    <Modal heading="The game has been found" onClose={onClose}>
      <div className={styles.container}>
        <p>
          To proceed, review the parameters of the gaming session and click the “Join” button. If applicable, you will
          need to pay the entry fee and required amount of gas immediately after clicking the “Join” button. After the
          end of the game, any unused gas will be refunded.
        </p>
        <div className={styles.info}>
          {items.map((item) => (
            <div className={styles.item} key={item.name}>
              <span className={styles.itemName}>{item.name}</span>
              <span className={styles.itemValue}>{item.value}</span>
            </div>
          ))}
        </div>
        <form className={styles.form} onSubmit={onJoinSubmit(handleJoinSession)}>
          <div className={cx(styles.input)}>
            <TextField
              theme="dark"
              label="Enter your name:"
              variant="active"
              maxLength={20}
              {...getJoinInputProps('name')}
            />
            <span className={cx(styles['field-error'])}>{joinErrors.name}</span>
          </div>
          <div className={styles.inputs}>
            <Button text="Cancel" color="dark" className={styles.button} onClick={onClose} />
            <Button type="submit" text="Join" className={styles.button} />
          </div>
        </form>
      </div>
    </Modal>
  );
}

export { GameFoundModal };
