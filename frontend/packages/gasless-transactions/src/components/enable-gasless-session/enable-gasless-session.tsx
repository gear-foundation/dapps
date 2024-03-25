import { Button, Checkbox } from '@gear-js/vara-ui';
import { useAccount } from '@gear-js/react-hooks';
import styles from './enable-gasless-session.module.css';
import { useGaslessTransactions } from '../..';
import { ReactComponent as GaslessSVG } from '../../assets/icons/gas-station-line.svg';
import { ReactComponent as PowerSVG } from '../../assets/icons/power.svg';
import { ChangeEvent } from 'react';

type Props = {
  type: 'button' | 'switcher';
  disabled?: boolean; // in case signless was enabled first
};

function EnableGaslessSession({ type, disabled }: Props) {
  const { account } = useAccount();
  const { isAvailable, isLoading, isEnabled, setIsEnabled } = useGaslessTransactions();

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
              color="light"
              className={styles.closeButton}
              onClick={handleDisableButtonClick}
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

            {!isAvailable && (
              <span className={styles.descr}>
                <span>Gas-free functionality is disabled at the moment.</span>
              </span>
            )}
          </div>
        </div>
      )}
    </>
  ) : null;
}

export { EnableGaslessSession };
