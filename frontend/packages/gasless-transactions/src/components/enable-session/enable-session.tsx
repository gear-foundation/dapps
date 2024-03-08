import { Button, Checkbox } from '@gear-js/vara-ui';
import styles from './enable-session.module.css';
import { useGaslessTransactions } from '../..';
import { ReactComponent as GaslessSVG } from '../../assets/icons/gas-station-line.svg';
import { ReactComponent as PowerSVG } from '../../assets/icons/power.svg';

type Props = {
  type: 'button' | 'switcher';
};

function EnableSession({ type }: Props) {
  const { isAvailable, isLoading, isActive, setIsActive } = useGaslessTransactions();

  const handleSwitcherChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.checked) {
      setIsActive(true);
    } else {
      setIsActive(false);
    }
  };

  const handleActivateSession = () => {
    setIsActive(true);
  };

  const handleDisableSession = () => {
    setIsActive(false);
  };

  return (
    <>
      {type === 'button' && (
        <>
          {isActive ? (
            <Button
              icon={PowerSVG}
              text="Disable"
              color="light"
              className={styles.closeButton}
              onClick={handleDisableSession}
            />
          ) : (
            <Button
              icon={GaslessSVG}
              color="transparent"
              text="Enable gasless transactions"
              disabled={!isAvailable || isLoading}
              className={styles.enableButton}
              onClick={handleActivateSession}
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
              disabled={!isAvailable || isLoading}
              checked={isActive}
              onChange={handleSwitcherChange}
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
  );
}

export { EnableSession };
