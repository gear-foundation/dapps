import React from 'react';
import { useNavigate } from 'react-router-dom';
import { hasLength, useForm } from '@mantine/form';
import { useApi } from '@gear-js/react-hooks';
import { Input, Select, Button } from '@gear-js/vara-ui';

import { useApp } from '@/app/context/ctx-app';
import { useGameMessage } from '@/app/hooks/use-game';

import { SpriteIcon } from '@/components/ui/sprite-icon';
import { useEzTransactions } from '@dapps-frontend/ez-transactions';
import { useCheckBalance } from '@dapps-frontend/hooks';

const initialValues = {
  bid: 0,
  level: 'Easy',
  username: '',
  tournamentName: '',
  duration: 10,
};

const validate = {
  username: hasLength({ min: 2, max: 25 }, 'Username must be 2-25 characters long'),
  tournamentName: hasLength({ min: 2, max: 25 }, 'Tournament name must be 2-25 characters long'),
};

const optionsLevel = [
  { value: 'Easy', label: 'Easy' },
  { value: 'Medium', label: 'Medium' },
  { value: 'Hard', label: 'Hardcore' },
];

const optionsDuration = [
  { value: 10, label: '10 min' },
  { value: 15, label: '15 min' },
  { value: 20, label: '20 min' },
  { value: 25, label: '25 min' },
  { value: 30, label: '30 min' },
];

export const TournamentCreate = () => {
  const { api } = useApi();
  const navigate = useNavigate();
  const { isPending, setIsPending } = useApp();

  const handleMessage = useGameMessage();
  const { gasless, signless } = useEzTransactions();
  const { checkBalance } = useCheckBalance({
    signlessPairVoucherId: signless.voucher?.id,
    gaslessVoucherId: gasless.voucherId,
  });
  const gasLimit = 120000000000;

  const form = useForm({
    initialValues: initialValues,
    validate,
    validateInputOnChange: true,
  });
  const { getInputProps, errors, reset } = form;

  const onSuccess = () => {
    setIsPending(false);
    reset();
    navigate('/');
  };
  const onError = () => {
    setIsPending(false);
  };

  const handleSubmit = form.onSubmit((values) => {
    setIsPending(true);
    const [decimals] = api?.registry.chainDecimals ?? [12];

    if (!gasless.isLoading) {
      checkBalance(gasLimit, () =>
        handleMessage({
          payload: {
            CreateNewTournament: {
              tournament_name: values.tournamentName,
              name: values.username,
              level: values.level,
              duration_ms: values.duration * 60000,
            },
          },
          voucherId: gasless.voucherId,
          gasLimit,
          value: (values.bid * 10 ** decimals).toString() || '0',
          onSuccess,
          onError,
        }),
      );
    }
  });

  return (
    <div className="flex flex-col justify-center items-center grow h-full">
      <h2 className="typo-h2">Create a private game</h2>
      <p>Create your own game tournament, invite your friends, and compete for the ultimate reward.</p>

      <form onSubmit={handleSubmit} className="grid gap-4 w-full max-w-[600px] mx-auto mt-5">
        <div className="flex flex-col gap-5">
          <div className="flex gap-5">
            <Input
              type="number"
              min={0}
              label="Entry fee"
              icon={() => <SpriteIcon name="vara-coin" height={24} width={24} />}
              {...getInputProps('bid')}
              required
              className="w-full"
            />

            <Select
              label="Difficulty level"
              options={optionsLevel}
              className="w-full"
              {...getInputProps('level')}
              required
            />
          </div>

          <Input
            type="text"
            label="Enter your name:"
            placeholder="Username"
            {...getInputProps('username')}
            required
            className="w-full"
          />

          <div className="flex gap-5">
            <Input
              type="text"
              label="Enter tournament name:"
              placeholder="Tournament name"
              {...getInputProps('tournamentName')}
              required
              className="w-full"
            />

            <Select
              label="Tournament  duration"
              options={optionsDuration}
              defaultValue={optionsDuration[0].value}
              className="w-full"
              {...getInputProps('duration')}
              required
            />
          </div>

          {/* <div className="rounded-2xl p-3 bg-[#F7F9FA]">
						<p>Required gas amount</p>
					</div> */}
        </div>
        <div className="flex gap-5">
          <Button color="grey" text="Back" className="w-full" onClick={() => navigate(-1)} isLoading={isPending} />
          <Button type="submit" text="Create game" className="w-full" isLoading={isPending} />
        </div>
      </form>
    </div>
  );
};
