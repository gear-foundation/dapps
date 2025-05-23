import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { ROUTES } from '@/app/consts';
import { BackIcon } from '@/assets/images';
import { Button, Input, Select, Slider } from '@/components';

import styles from './create-game.module.scss';

type FormData = {
  name: string;
  players: number;
  time: number;
  buyIn: number;
};

const initialFormData: FormData = {
  name: '',
  players: 9,
  time: 60,
  buyIn: 15000,
};

const buyInOptions = [
  { value: 2000, label: '2k' },
  { value: 5000, label: '5k' },
  { value: 10000, label: '10k' },
  { value: 15000, label: '15k' },
  { value: 50000, label: '50k' },
  { value: 100000, label: '100k' },
];

const playerOptions = [
  { value: 9, label: '9 players' },
  { value: 8, label: '8 players' },
  { value: 7, label: '7 players' },
  { value: 6, label: '6 players' },
  { value: 5, label: '5 players' },
  { value: 4, label: '4 players' },
  { value: 3, label: '3 players' },
  { value: 2, label: '2 players' },
];

const timeOptions = [
  { value: 15, label: '15 sec' },
  { value: 30, label: '30 sec' },
  { value: 60, label: '60 sec' },
  { value: 120, label: '120 sec' },
];

export default function CreateLobby() {
  const [formData, setFormData] = useState<FormData>(initialFormData);
  const [isLoading, setIsLoading] = useState(false);
  const navigate = useNavigate();

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

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);

    try {
      console.log('Creating game with settings:', formData);

      await new Promise((resolve) => setTimeout(resolve, 1500));

      navigate(ROUTES.GAME);
    } catch (error) {
      console.error('Error creating game:', error);
    } finally {
      setIsLoading(false);
    }
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

          <Select
            name="players"
            value={formData.players}
            options={playerOptions}
            onChange={(value) => handleSelectChange('players', value)}
          />

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
