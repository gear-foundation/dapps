import { useAccount } from '@gear-js/react-hooks';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { BIG_BLIND, ROUTES, SMALL_BLIND } from '@/app/consts';
import { BackIcon } from '@/assets/images';
import { Button, Input, Slider } from '@/components';
import { useUserName } from '@/features/game/hooks';
import { LobbyCreatedPayload, useCreateLobbyMessage, useEventLobbyCreatedSubscription } from '@/features/game/sails';
import { useKeys } from '@/features/zk/hooks';
import { getPkBytes } from '@/features/zk/utils';

import styles from './create-game.module.scss';

type FormData = {
  name: string;
  time: number;
  buyIn: number;
};

const initialFormData: FormData = {
  name: '',
  time: 60,
  buyIn: 5000,
};

const buyInOptions = [
  { value: 2000, label: '2k' },
  { value: 5000, label: '5k' },
  { value: 10000, label: '10k' },
  { value: 15000, label: '15k' },
  { value: 50000, label: '50k' },
  { value: 100000, label: '100k' },
];

const timeOptions = [
  { value: 15, label: '15 sec' },
  { value: 30, label: '30 sec' },
  { value: 60, label: '60 sec' },
  { value: 120, label: '120 sec' },
];

function CreateGame() {
  const [formData, setFormData] = useState<FormData>(initialFormData);
  const [isLoading, setIsLoading] = useState(false);
  const { account } = useAccount();
  const navigate = useNavigate();
  const { userName } = useUserName();
  const { pk } = useKeys();

  const onLobbyCreated = (payload: LobbyCreatedPayload) => {
    setIsLoading(false);
    navigate(ROUTES.GAME.replace(':gameId', payload.lobby_address));
  };

  useEventLobbyCreatedSubscription({ onData: onLobbyCreated });
  const { createLobbyMessage } = useCreateLobbyMessage();

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type, checked } = e.target;
    setFormData({
      ...formData,
      [name]: type === 'checkbox' ? checked : value,
    });
  };

  const handleSelectChange = (name: string, value: string | number) => {
    setFormData({
      ...formData,
      [name]: value,
    });
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    if (!account) return;

    void createLobbyMessage(
      {
        config: {
          time_per_move_ms: formData.time * 1000,
          admin_id: account.decodedAddress,
          admin_name: userName,
          big_blind: BIG_BLIND,
          lobby_name: formData.name,
          small_blind: SMALL_BLIND,
          starting_bank: formData.buyIn,
        },
        pk: getPkBytes(pk),
      },
      {
        onError: (error) => {
          console.error('Error creating game:', error);
          setIsLoading(false);
        },
      },
    );
  };

  const handleCancel = () => {
    navigate(ROUTES.HOME);
  };

  return (
    <>
      <div className={styles.container}>
        <div className={styles.header}>
          <Button color="contrast" rounded className={styles.backButton} onClick={handleCancel}>
            <BackIcon />
          </Button>
          <h1 className={styles.title}>Create lobby</h1>
        </div>

        <p className={styles.description}>
          When creating a password, your lobby will automatically be marked as private. Leave it blank to create an open
          lobby accessible to all
        </p>

        <form className={styles.form} onSubmit={handleSubmit}>
          <Input name="name" value={formData.name} onChange={handleInputChange} placeholder="Lobby name" required />

          {/* TODO: add password in next iterations */}
          {/* <div className={styles.passwordContainer}>
            <Input
              name="password"
              value={formData.password}
              onChange={handleInputChange}
              placeholder="Password (Optional)"
              type={showPassword ? 'text' : 'password'}
            />
            <Button
              color="transparent"
              onClick={() => setShowPassword(!showPassword)}
              className={styles.showPasswordButton}>
              {showPassword ? <HidePasswordIcon /> : <ShowPasswordIcon />}
            </Button>
          </div> */}

          <Slider
            label="Time per move"
            options={timeOptions}
            defaultValue={initialFormData.time}
            onChange={(value) => handleSelectChange('time', value)}
          />

          <Slider
            label="Buy-in (entry fee in PTS)"
            options={buyInOptions}
            defaultValue={initialFormData.buyIn}
            onChange={(value) => handleSelectChange('buyIn', value)}
          />

          <div className={styles.actions}>
            <Button color="contrast" onClick={handleCancel}>
              Cancel
            </Button>
            <Button type="submit" disabled={isLoading}>
              Create lobby
            </Button>
          </div>
        </form>
      </div>
    </>
  );
}

export default CreateGame;
