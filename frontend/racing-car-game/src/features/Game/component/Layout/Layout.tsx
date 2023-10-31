import { memo, useCallback, useEffect, useRef, useState } from 'react';
import { useNavigate } from 'react-router';
import { useAtom } from 'jotai';
import isEqual from 'lodash.isequal';
import { useAccount, useAlert, useApi, useHandleCalculateGas } from '@gear-js/react-hooks';
import { Bytes } from '@polkadot/types';
import { UnsubscribePromise } from '@polkadot/api/types';
import { UserMessageSent } from '@gear-js/api';
import styles from './Layout.module.scss';
import { cx, logger, withoutCommas } from '@/utils';
import { Heading } from '../Heading';
import { Road } from '../Road';
import { Button } from '@/ui';
import accelerateSVG from '@/assets/icons/accelerate-icon.svg';
import shootSVG from '@/assets/icons/shoot-icon.svg';
import { ReactComponent as GearLogoIcon } from '@/assets/icons/gear-logo-icon.svg';
import { CURRENT_GAME, IS_SUBSCRIBED_ATOM } from '@/atoms';
import { usePlayerMoveMessage, useStartGameMessage } from '../../hooks';
import { Footer, Loader } from '@/components';
import { MessageDetails, RepliesQueue, UserMessage, WinStatus } from './Layout.interface';
import { PLAY } from '@/App.routes';
import { ContractError, DecodedReply, DecodedReplyItem, GameState } from '@/types';
import { ADDRESS } from '@/consts';
import { useCheckBalance } from '@/hooks';
import { useAccountAvailableBalance } from '@/features/Wallet/hooks';
import { CURRENT_SENT_MESSAGE_ID_ATOM, IS_STATE_READ_ATOM, REPLY_DATA_ATOM } from '../../atoms';

function LayoutComponent() {
  const [currentGame, setCurrentGame] = useAtom(CURRENT_GAME);
  const [isPlayerAction, setIsPlayerAction] = useState<boolean>(true);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isRoadLoaded, setIsRoadLoaded] = useState(false);
  const { isAvailableBalanceReady } = useAccountAvailableBalance();
  const { account } = useAccount();
  const alert = useAlert();
  const { checkBalance } = useCheckBalance();
  const navigate = useNavigate();
  const sendPlayerMoveMessage = usePlayerMoveMessage();
  const { meta, message: startGameMessage } = useStartGameMessage();
  const calculateGas = useHandleCalculateGas(ADDRESS.CONTRACT, meta);
  const [isStateRead, setIsStateRead] = useAtom(IS_STATE_READ_ATOM);
  const { api } = useApi();

  const messageSubscription: React.MutableRefObject<UnsubscribePromise | null> = useRef(null);
  const repliesQueue: React.MutableRefObject<RepliesQueue> = useRef([]);
  const [replyData, setReplyData] = useAtom(REPLY_DATA_ATOM);
  const [currentSentMessageId, setCurrentSentMessageId] = useAtom(CURRENT_SENT_MESSAGE_ID_ATOM);
  const [isSubscribed, setIsSubscribed] = useAtom(IS_SUBSCRIBED_ATOM);

  const getDecodedPayload = (payload: Bytes) => {
    if (meta?.types.others.output) {
      return meta.createType(meta?.types.others.output, payload).toHuman();
    }
  };

  const getDecodedReply = (payload: Bytes): DecodedReply => {
    const decodedPayload = getDecodedPayload(payload);

    return decodedPayload as DecodedReply;
  };

  const handleUnsubscribeFromEvent = (onSuccess?: () => void) => {
    if (messageSubscription.current) {
      messageSubscription.current?.then((unsubCallback) => {
        unsubCallback();
        logger('UNsubscribed from reply');
        setIsSubscribed(false);
        onSuccess?.();
      });
    }
  };

  const decodePair = useCallback(() => {
    logger('triggers SentMessageId Effect');
    if (currentSentMessageId) {
      logger(`SentMessageId exists: ${currentSentMessageId}`);
      logger(repliesQueue.current);
      const foundRepliesPair = repliesQueue.current.find(
        (item) => (item.auto?.toHuman().details as MessageDetails).to === currentSentMessageId,
      );

      logger(`Reply Pair found:`);
      logger({ auto: foundRepliesPair?.auto?.toHuman(), manual: foundRepliesPair?.manual?.toHuman() });
      logger(`Reply found: ${foundRepliesPair?.manual}`);

      if (foundRepliesPair?.auto && foundRepliesPair.manual) {
        const { manual } = foundRepliesPair;

        logger('trying to decode....:');
        try {
          const reply = getDecodedReply(manual.payload);
          logger('DECODED message successfully');
          logger('new reply HAS COME:');
          logger(reply);

          if (reply && reply.cars.length && !isEqual(reply?.cars, replyData?.cars)) {
            logger('prev reply state:');
            logger(replyData);
            logger('new reply UPDATED and going to state:');
            logger(reply);
            setReplyData(reply);
            setCurrentSentMessageId(null);
            handleUnsubscribeFromEvent();
          }
        } catch (e) {
          logger(e);
          alert.error((e as ContractError).message);
        }
      }

      if (foundRepliesPair && foundRepliesPair.auto && !foundRepliesPair.manual) {
        setCurrentSentMessageId(null);
        setIsPlayerAction(true);
      }

      if (!foundRepliesPair?.auto) {
        setTimeout(decodePair, 1500);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [currentSentMessageId]);

  useEffect(() => {
    decodePair();
  }, [decodePair]);

  const handleChangeState = ({ data: _data }: UserMessageSent) => {
    const { message } = _data;

    const { destination, source, details: messageDetails } = message as UserMessage;

    const isOwner = destination.toHex() === account?.decodedAddress;
    const isCurrentProgram = source.toHex() === ADDRESS.CONTRACT;

    const details = messageDetails.toHuman() as MessageDetails;

    if (isOwner && isCurrentProgram) {
      if (
        details?.to &&
        !repliesQueue.current.map((item) => (item.auto?.toHuman().details as MessageDetails).to).includes(details?.to)
      ) {
        console.log('pushed');
        console.log(repliesQueue.current.map((item) => (item.auto?.toHuman()?.details as MessageDetails).to));
        repliesQueue.current.push({ auto: message, manual: null });
      }

      if (!details && !repliesQueue.current[repliesQueue.current.length - 1].manual) {
        console.log('pushed2');
        repliesQueue.current[repliesQueue.current.length - 1].manual = message;
      }
      logger(repliesQueue.current.map((item) => ({ auto: item.auto?.toHuman(), manual: item.manual?.toHuman() })));
    }
  };
  // eslint-disable-next-line react-hooks/exhaustive-deps

  const handleSubscribeToEvent = useCallback(async () => {
    if (api && meta && !isSubscribed) {
      messageSubscription.current = api.gearEvents.subscribeToGearEvent('UserMessageSent', handleChangeState);
      setIsSubscribed(true);
      logger('Subscribed on reply');
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, isSubscribed, meta]);

  const defineStrategyAction = (type: 'accelerate' | 'shoot') => {
    if (type === 'accelerate') {
      return 'BuyAcceleration';
    }

    if (type === 'shoot') {
      return 'BuyShell';
    }
  };

  const handleActionChoose = (type: 'accelerate' | 'shoot') => {
    setIsPlayerAction(false);
    logger(`CLICK ACTION ${type}`);
    logger(`Disabling actions`);
    const payload = {
      PlayerMove: {
        strategy_action: defineStrategyAction(type),
      },
    };

    handleSubscribeToEvent();

    calculateGas(payload)
      .then((res) => res.toHuman())
      .then(({ min_limit }) => {
        const limit = withoutCommas(min_limit as string);
        logger(`Calculating gas:`);
        logger(`MIN_LIMIT ${min_limit}`);
        logger(`LIMIT ${Math.floor(Number(limit) + Number(limit) * 0.2)}`);
        logger(`Calculated gas SUCCESS`);
        logger(`Sending message`);
        console.log(`START TURN ${Number(currentGame?.currentRound) + 1}`);

        sendPlayerMoveMessage({
          payload,
          gasLimit: Math.floor(Number(limit) + Number(limit) * 0.2),
          onError: () => {
            setIsPlayerAction(true);
            handleUnsubscribeFromEvent();
            logger(`Errror send message`);
          },
          onSuccess: (messageId) => {
            logger(`sucess on ID: ${messageId}`);
          },
          onInBlock: (messageId) => {
            logger('messageInBlock');
            logger(`messageID: ${messageId}`);
            setCurrentSentMessageId(messageId);
          },
        });
      })
      .catch((error) => {
        logger(error);
        alert.error('Gas calculation error');
      });
  };

  const defineWinStatus = (): WinStatus => {
    if (currentGame?.state === 'Finished') {
      return currentGame.result;
    }

    return null;
  };

  const handleStartNewGame = useCallback(
    (startManually?: boolean) => {
      if (meta && (!currentGame || startManually)) {
        setIsPlayerAction(false);
        setIsLoading(true);
        const payload = {
          StartGame: null,
        };

        calculateGas(payload)
          .then((res) => res.toHuman())
          .then(({ min_limit }) => {
            const limit = withoutCommas(min_limit as string);

            startGameMessage({
              payload,
              gasLimit: Math.floor(Number(limit) + Number(limit) * 0.2),
              onSuccess: () => {
                setIsStateRead(false);
                setIsLoading(false);
              },
              onError: () => {
                logger('error');
                navigate(PLAY, { replace: true });
              },
            });
          })
          .catch((error) => {
            logger(error);
            alert.error('Gas calculation error');
            navigate(PLAY, { replace: true });
          });
      }
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [meta, currentGame],
  );

  useEffect(() => {
    if (isStateRead) {
      setIsPlayerAction(true);
    }
  }, [isStateRead]);

  useEffect(() => {
    checkBalance(handleStartNewGame, () => {
      navigate(PLAY, { replace: true });
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [handleStartNewGame]);

  useEffect(() => {
    if (replyData && currentGame) {
      const { cars, result } = replyData;
      logger('Updates state to new reply');

      setCurrentGame(() =>
        cars.reduce((acc: GameState, item: DecodedReplyItem) => {
          const [address, position, effect] = item;

          return {
            ...acc,
            cars: {
              ...acc.cars,
              [address]: {
                ...acc.cars[address],
                position,
                roundResult: effect,
              },
            },
          };
        }, currentGame),
      );
      setCurrentGame((prev) =>
        prev
          ? {
              ...prev,
              result,
              state: result ? 'Finished' : prev.state,
              currentRound: String(Number(prev.currentRound) + 1),
            }
          : null,
      );
      logger('Enabling actions');
      setIsPlayerAction(true);
      logger(`END OF TURN ${Number(currentGame.currentRound) + 1}`);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [replyData]);

  const handleRoadLoaded = () => {
    setIsRoadLoaded(true);
  };

  return (
    <>
      {currentGame && account?.decodedAddress && isAvailableBalanceReady && !isLoading && isStateRead ? (
        <div className={cx(styles.container, currentGame.state !== 'Finished' ? styles['container-flexed'] : '')}>
          {isRoadLoaded && (
            <Heading
              currentTurn={currentGame.currentRound}
              isPlayerAction={isPlayerAction}
              winStatus={defineWinStatus()}
            />
          )}
          <Road newCars={currentGame.cars} carIds={currentGame.carIds} onRoadLoaded={handleRoadLoaded} />
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
                    isLoading={!account.decodedAddress || !meta}
                    className={cx(styles['control-button'])}
                    onClick={() => checkBalance(() => handleActionChoose('accelerate'))}
                  />
                  <Button
                    label="Shoot"
                    variant="primary"
                    size="large"
                    icon={shootSVG}
                    disabled={!isPlayerAction}
                    isLoading={!account.decodedAddress || !meta}
                    className={cx(styles['control-button'], styles['control-button-red'])}
                    onClick={() => checkBalance(() => handleActionChoose('shoot'))}
                  />
                </div>
              )}
              {currentGame.state === 'Finished' && (
                <div className={cx(styles['rewards-wrapper'])}>
                  <Button
                    variant="primary"
                    label="Play again"
                    size="large"
                    className={cx(styles.btn)}
                    onClick={() => checkBalance(() => handleStartNewGame(true))}
                  />
                </div>
              )}
              {currentGame.state !== 'Finished' && (
                <>
                  <div className={cx(styles.footer)}>
                    <Footer />
                  </div>
                </>
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
                  <Footer />
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
