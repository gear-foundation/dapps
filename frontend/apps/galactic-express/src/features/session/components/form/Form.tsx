import { useAtomValue, useSetAtom, useAtom } from 'jotai';
import { CURRENT_GAME_ATOM, IS_LOADING, PLAYER_NAME_ATOM } from 'atoms';
import { useAccount, withoutCommas } from '@gear-js/react-hooks';
import { Button } from '@gear-js/ui';
import { useForm } from '@mantine/form';
import { Card } from 'components';
import { ChangeEvent, Dispatch, SetStateAction, useState } from 'react';
import { ReactComponent as RocketSVG } from '../../assets/rocket.svg';
import { INITIAL_VALUES, VALIDATE, WEATHERS } from '../../consts';
import { useLaunchMessage } from '../../hooks';
import { Range } from '../range';
import { Probability } from '../probability';
import styles from './Form.module.scss';

type Props = {
  weather: string;
  bid: string | undefined;
  isAdmin: boolean;
  setRegistrationStatus: Dispatch<
    SetStateAction<'registration' | 'success' | 'error' | 'NotEnoughParticipants' | 'MaximumPlayersReached'>
  >;
};

function Form({ weather, bid, isAdmin, setRegistrationStatus }: Props) {
  const { account } = useAccount();
  const [isLoading, setIsLoading] = useAtom(IS_LOADING);
  const setCurrentGame = useSetAtom(CURRENT_GAME_ATOM);
  const { values, getInputProps, onSubmit, setFieldValue } = useForm({
    initialValues: { ...INITIAL_VALUES },
    validate: VALIDATE,
  });
  const playerName = useAtomValue(PLAYER_NAME_ATOM);
  const currentGameAddress = useAtomValue(CURRENT_GAME_ATOM);

  const { fuel, payload } = values;

  const { meta, message: sendMessage } = useLaunchMessage();

  const handleNumberInputChange = ({ target }: ChangeEvent<HTMLInputElement>) => {
    const value = +target.value;
    const min = +target.min || -Infinity;
    const max = +target.max || Infinity;

    const rangeValue = Math.max(min, Math.min(max, value));

    setFieldValue(target.name, String(rangeValue));
  };

  const getNumberInputProps = (name: keyof typeof values) => ({
    ...getInputProps(name),
    name, // passing name cuz getInputProps doesn't do this
    onChange: handleNumberInputChange,
  });

  const handleSubmit = () => {
    if (!isAdmin && meta && account?.decodedAddress) {
      setIsLoading(true);
      sendMessage({
        payload: {
          Register: {
            creator: currentGameAddress,
            participant: { fuel_amount: fuel, payload_amount: payload, name: playerName, id: account.decodedAddress },
          },
        },
        value: Number(withoutCommas(bid || '')),
        onSuccess: () => {
          setRegistrationStatus('success');
          setCurrentGame('');
          setIsLoading(false);
        },
        onError: () => {
          setIsLoading(false);
        },
      });
    }

    if (isAdmin && meta) {
      sendMessage({ payload: { StartGame: { fuel_amount: fuel, payload_amount: payload } } });
    }
  };

  return (
    <form onSubmit={onSubmit(handleSubmit)}>
      <Card className={styles.calculation}>
        <h3 className={styles.heading}>Calculation Block</h3>

        <div className={styles.ranges}>
          <Range key="first_range" label="Payload:" {...getNumberInputProps('payload')} />
          <Range key="second_range" label="Fuel:" {...getNumberInputProps('fuel')} />
        </div>

        <footer className={styles.footer}>
          <Probability weather={WEATHERS[weather as keyof typeof WEATHERS].weight} payload={+payload} fuel={+fuel} />
          <Button
            type="submit"
            icon={RocketSVG}
            text={isAdmin ? 'Launch rocket and start Game' : 'Launch Rocket'}
            disabled={isLoading}
            color="lightGreen"
          />
        </footer>
      </Card>
    </form>
  );
}

export { Form };
