import { memo, useCallback, useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import isEqual from 'lodash.isequal';
import { useAccount } from '@gear-js/react-hooks';

import { Container, Footer } from '@dapps-frontend/ui';
import { useEzTransactions } from 'gear-ez-transactions';
import styles from './Layout.module.scss';
import { cx } from '@/utils';
import { Heading } from '../Heading';
import { Road } from '../Road';
import { Button } from '@/ui';
import accelerateSVG from '@/assets/icons/accelerate-icon.svg';
import shootSVG from '@/assets/icons/shoot-icon.svg';
import GearLogoIcon from '@/assets/icons/gear-logo-icon.svg?react';
import { Loader } from '@/components';
import { PLAY } from '@/App.routes';
import { useAccountAvailableBalance } from '@/features/Wallet/hooks';
import { useEventRoundInfoSubscription, usePlayerMoveMessage, useStartGameMessage, useGameQuery } from '../../sails';
import { GameResult } from '@/app/utils';

function LayoutComponent() {
  const { signless, gasless } = useEzTransactions();
  const { game: currentGame, refetch } = useGameQuery();
  const isCurrentGameRead = currentGame !== undefined;
  const [isPlayerAction, setIsPlayerAction] = useState<boolean>(true);
  const [isLoading, setIsLoading] = useState(false);
  const [isRoadLoaded, setIsRoadLoaded] = useState(false);
  const { isAvailableBalanceReady } = useAccountAvailableBalance();
  const { account } = useAccount();
  const navigate = useNavigate();
  const { playerMoveMessage } = usePlayerMoveMessage();
  const { startGameMessage } = useStartGameMessage();

  const subscriptionCallback = useCallback(() => {
    refetch();
    setIsPlayerAction(true);
  }, []);

  useEventRoundInfoSubscription(subscriptionCallback);

  const handleActionChoose = async (type: 'accelerate' | 'shoot') => {
    setIsPlayerAction(false);

    const strategyActionMap = { accelerate: 'BuyAcceleration' as const, shoot: 'BuyShell' as const };
    playerMoveMessage(strategyActionMap[type], { onError: () => setIsPlayerAction(true) });
  };

  const defineWinStatus = (): GameResult | null => {
    if (currentGame?.state === 'Finished') {
      return currentGame.result;
    }

    return null;
  };

  const handleStartNewGame = useCallback(
    async (startManually?: boolean) => {
      console.log('handleStartNewGame', isCurrentGameRead && !isLoading && (!currentGame || startManually));
      if (isCurrentGameRead && !isLoading && (!currentGame || startManually)) {
        const onError = () => {
          setIsLoading(false);
          navigate(PLAY, { replace: true });
        };

        setIsPlayerAction(true);
        setIsLoading(true);

        await startGameMessage({ onError });
        console.log('refetch');
        await refetch();
        setIsLoading(false);
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [currentGame, isCurrentGameRead, account, gasless, signless],
  );

  useEffect(() => {
    handleStartNewGame();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isCurrentGameRead]);

  const handleRoadLoaded = () => {
    setIsRoadLoaded(true);
  };

  return (
    <>
      {currentGame && account?.decodedAddress && isAvailableBalanceReady && !isLoading ? (
        <div className={cx(styles.container, currentGame.state !== 'Finished' ? styles['container-flexed'] : '')}>
          {isRoadLoaded && (
            <Heading
              currentTurn={String(currentGame.current_round)}
              isPlayerAction={isPlayerAction}
              winStatus={defineWinStatus()}
            />
          )}
          <Road newCars={currentGame.cars} carIds={currentGame.car_ids} onRoadLoaded={handleRoadLoaded} />
          {isRoadLoaded && (
            <>
              {currentGame.state !== 'Finished' && (
                <div className={cx(styles.controls)}>
                  <Button
                    label="Accelerate"
                    variant="primary"
                    size="large"
                    icon={accelerateSVG}
                    disabled={!isPlayerAction}
                    isLoading={!account.decodedAddress}
                    className={cx(styles['control-button'])}
                    onClick={() => handleActionChoose('accelerate')}
                  />
                  <Button
                    label="Shoot"
                    variant="primary"
                    size="large"
                    icon={shootSVG}
                    disabled={!isPlayerAction}
                    isLoading={!account.decodedAddress}
                    className={cx(styles['control-button'], styles['control-button-red'])}
                    onClick={() => handleActionChoose('shoot')}
                  />
                </div>
              )}
              {currentGame.state === 'Finished' && (
                <div className={cx(styles['rewards-wrapper'])}>
                  <Button
                    variant="primary"
                    label="Play again"
                    size="large"
                    isLoading={gasless.isLoading}
                    className={cx(styles.btn)}
                    onClick={() => handleStartNewGame(true)}
                  />
                </div>
              )}
              {currentGame.state !== 'Finished' && (
                <Container className={cx(styles.footer)}>
                  <Footer vara />
                </Container>
              )}
              {currentGame.state === 'Finished' && (
                <div className={cx(styles['footer-wrapper'])}>
                  <div className={styles.banner}>
                    <div className={styles.banner__right}>
                      <h2 className={styles.banner__title}>
                        Thank you for your interest <span>in the Vara Network.</span>
                      </h2>
                      <div className={styles.banner__text}>
                        <p>You&apos;ve experienced a fully on-chain game.</p>
                        <p>
                          We look forward to having you join the ranks of developers shaping the new generation of
                          decentralized applications.
                        </p>
                      </div>
                    </div>
                    <ul className={styles.banner__left}>
                      <li className={styles.banner__item}>
                        <div className={styles.banner__icon}>
                          <GearLogoIcon width={24} height={24} />
                        </div>
                        <p className={styles['visit-block']}>
                          Visit the{' '}
                          <a href="https://wiki.gear-tech.io/" target="_blank" rel="noreferrer">
                            Gear Wiki
                          </a>{' '}
                          to discover how easy it is to create programs using the Gear Protocol.
                        </p>
                      </li>
                      <li className={styles.banner__item}>
                        <div className={styles.banner__icon}>
                          <GearLogoIcon width={24} height={24} />
                        </div>
                        <p className={styles['visit-block']}>
                          Consider enrolling in a free course at{' '}
                          <a href="https://academy.gear.foundation/" target="_blank" rel="noreferrer">
                            Gear&nbsp;Academy
                          </a>{' '}
                          to become a top-notch Web3 developer.
                        </p>
                      </li>
                    </ul>
                  </div>
                  <Container>
                    <Footer vara />
                  </Container>
                </div>
              )}
            </>
          )}
        </div>
      ) : (
        <div className={styles['loader-wrapper']}>
          <Loader />
        </div>
      )}
    </>
  );
}

const Layout = memo(LayoutComponent, isEqual);

export { Layout };
