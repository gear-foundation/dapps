import { useAccount } from '@gear-js/react-hooks';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { Switcher } from '@dapps-frontend/ui';

import { ROUTES, UI_TIME_COVER_MS } from '@/app/consts';
import { BackIcon } from '@/assets/images';
import { Button, Input, Slider } from '@/components';
import { useUserName } from '@/features/game/hooks';
import { LobbyCreatedPayload, useCreateLobbyMessage, useEventLobbyCreatedSubscription } from '@/features/game/sails';
import { useZkKeys } from '@/features/zk/hooks';
import { getPkBytes } from '@/features/zk/utils';

import styles from './create-game.module.scss';

const ONE_HOUR_MS = 60 * 60 * 1000;

type FormData = {
  name: string;
  time: number;
  buyIn: number;
  revival: boolean;
  lobbyTimeLimitMs: number;
  timeUntilStartMinutes: number;
};

const initialFormData: FormData = {
  name: '',
  time: 60,
  buyIn: 5000,
  revival: false,
  lobbyTimeLimitMs: ONE_HOUR_MS,
  timeUntilStartMinutes: 0,
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

const LOBBY_TIME_LIMIT_OPTIONS = [
  { value: ONE_HOUR_MS / 2, label: '30 min' },
  { value: ONE_HOUR_MS, label: '1 hour' },
  { value: 2 * ONE_HOUR_MS, label: '2 hours' },
  { value: 0, label: 'No limit' },
];

function CreateGame() {
  const [formData, setFormData] = useState<FormData>(initialFormData);
  const { account } = useAccount();
  const navigate = useNavigate();
  const { userName } = useUserName();
  const { pk } = useZkKeys();

  const onLobbyCreated = (payload: LobbyCreatedPayload) => {
    navigate(ROUTES.GAME.replace(':gameId', payload.lobby_address));
  };

  useEventLobbyCreatedSubscription({ onData: onLobbyCreated });
  const { createLobbyMessage, isPending } = useCreateLobbyMessage();

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

    void createLobbyMessage({
      config: {
        time_per_move_ms: formData.time * 1000 + UI_TIME_COVER_MS,
        admin_id: account.decodedAddress,
        admin_name: userName,
        lobby_name: formData.name,
        starting_bank: formData.buyIn,
        revival: formData.revival,
        lobby_time_limit_ms: formData.lobbyTimeLimitMs === 0 ? null : formData.lobbyTimeLimitMs,
        time_until_start_ms:
          formData.timeUntilStartMinutes != null && formData.timeUntilStartMinutes > 0
            ? formData.timeUntilStartMinutes * 60 * 1000
            : null,
      },
      pk: getPkBytes(pk),
    });
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

        <form className={styles.form} onSubmit={handleSubmit}>
          <Input name="name" value={formData.name} onChange={handleInputChange} placeholder="Lobby name" required />

          <Slider
            label="Time per move"
            options={timeOptions}
            defaultValue={initialFormData.time}
            onChange={(value) => handleSelectChange('time', value)}
          />

          <Slider
            label="Lobby time limit"
            options={LOBBY_TIME_LIMIT_OPTIONS}
            defaultValue={initialFormData.lobbyTimeLimitMs}
            onChange={(value) => handleSelectChange('lobbyTimeLimitMs', value)}
          />

          <Slider
            label="Buy-in (entry fee in PTS)"
            options={buyInOptions}
            defaultValue={initialFormData.buyIn}
            onChange={(value) => handleSelectChange('buyIn', value)}
          />

          <div className={styles.switcherRow}>
            <div className={styles.switcherLabelRow}>
              <span className={styles.switcherLabel}>Allow retired players</span>
            </div>
            <Switcher checked={formData.revival} onChange={(v) => setFormData({ ...formData, revival: v })} />
          </div>

          <div className={styles.startDelayBlock}>
            <label className={styles.startDelayLabel} htmlFor="timeUntilStartMinutes">
              Start in (minutes)
            </label>
            <Input
              id="timeUntilStartMinutes"
              name="timeUntilStartMinutes"
              type="number"
              className={styles.startDelayInput}
              min={0}
              value={formData.timeUntilStartMinutes === 0 ? '' : formData.timeUntilStartMinutes}
              onChange={(e) => {
                const raw = e.target.value;
                const num = raw === '' ? 0 : parseInt(raw, 10);
                setFormData({
                  ...formData,
                  timeUntilStartMinutes: Number.isNaN(num) || num < 0 ? 0 : num,
                });
              }}
              placeholder="0 = immediately"
            />
          </div>

          <div className={styles.actions}>
            <Button color="contrast" onClick={handleCancel} disabled={isPending}>
              Cancel
            </Button>
            <Button type="submit" disabled={isPending}>
              Create lobby
            </Button>
          </div>
        </form>
      </div>
    </>
  );
}

export default CreateGame;
