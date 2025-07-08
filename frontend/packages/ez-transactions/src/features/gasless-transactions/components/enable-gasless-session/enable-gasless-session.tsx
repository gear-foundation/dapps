import { useAccount } from '@gear-js/react-hooks';
import { Button, Checkbox } from '@gear-js/vara-ui';
import { ChangeEvent } from 'react';

import GaslessSVG from '@ez/assets/icons/gas-station-line.svg?react';
import PowerSVG from '@ez/assets/icons/power.svg?react';

import { useGaslessTransactions } from '../../context';

import styles from './enable-gasless-session.module.css';

type Props = {
  type: 'button' | 'switcher';
  disabled?: boolean;
  message?: string;
};

function EnableGaslessSession({ type, disabled, message }: Props) {
  const { account } = useAccount();
  const { voucherStatus, isLoading, isEnabled, setIsEnabled } = useGaslessTransactions();

  const isAvailable = voucherStatus?.enabled;

  const handleSwitchChange = ({ target }: ChangeEvent<HTMLInputElement>) => setIsEnabled(target.checked);

  const handleEnableButtonClick = () => setIsEnabled(true);
  const handleDisableButtonClick = () => setIsEnabled(false);

  return account ? (
    <>
      {type === 'button' && (
        <>
          {isEnabled ? (
            <Button
              icon={PowerSVG}
              text="Disable"
              color="grey"
              className={styles.closeButton}
              onClick={handleDisableButtonClick}
              disabled={disabled}
            />
          ) : (
            <Button
              icon={GaslessSVG}
              color="transparent"
              text="Enable gasless transactions"
              disabled={disabled || !isAvailable || isLoading}
              className={styles.enableButton}
              onClick={handleEnableButtonClick}
            />
          )}
        </>
      )}

      {type === 'switcher' && (
        <div className={styles.switchContainer}>
          <div className={styles.switcherWrapper}>
            <Checkbox
              label=""
              type="switch"
              disabled={disabled || !isAvailable || isLoading}
              checked={isEnabled}
              onChange={handleSwitchChange}
            />
          </div>

          <div className={styles.contentWrapper}>
            <div className={styles.headingWrapper}>
              <GaslessSVG />
              <span className={styles.heading}>Enable gasless</span>
              {isLoading && <span className={styles.loader} />}
            </div>

            {!isLoading && (!isAvailable || message) && (
              <span className={styles.descr}>
                <span>{!isAvailable ? 'Gas-free functionality is disabled at the moment.' : message}</span>
              </span>
            )}
          </div>
        </div>
      )}
    </>
  ) : null;
}

export { EnableGaslessSession };
