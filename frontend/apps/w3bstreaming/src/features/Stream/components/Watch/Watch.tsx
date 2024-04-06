import { MutableRefObject, useEffect, useRef, useState, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { web3Enable, web3FromAddress } from '@polkadot/extension-dapp';
import { SignerResult } from '@polkadot/api/types';
import { stringToHex } from '@polkadot/util';
import { useAccount } from '@gear-js/react-hooks';
import styles from './Watch.module.scss';
import { cx } from '@/utils';
import { CandidateMsg, ErrorMsg, OfferMsg, StreamState, WatchProps } from './Watch.interface';
import { Player } from '../Player';
import { Loader } from '@/components';
import { RTC_CONFIG } from '../../config';
import { Button } from '@/ui';
import { useProgramState } from '@/hooks';

function Watch({ socket, streamId }: WatchProps) {
  const navigate = useNavigate();
  const publicKey: MutableRefObject<SignerResult | null> = useRef(null);
  const retryIntervalId: MutableRefObject<ReturnType<typeof setInterval> | null> = useRef(null);
  const remoteVideo: MutableRefObject<HTMLVideoElement | null> = useRef(null);
  const [localStream, setLocalStream] = useState<MediaStream | null>(null);
  const peerConnection: MutableRefObject<RTCPeerConnection | null> = useRef(null);
  const { account } = useAccount();
  const {
    state: { streamTeasers },
  } = useProgramState();
  const [streamStatus, setStreamStatus] = useState<StreamState>('ready-to-play');

  const handleGetPublicKey = async () => {
    if (account?.address && !publicKey.current) {
      const { address } = account;

      web3Enable('streaming');

      try {
        const { signer } = await web3FromAddress(address);

        if (!signer.signRaw) {
          throw new Error('signRaw does not exist');
        }

        publicKey.current = await signer.signRaw({ address, data: stringToHex(address), type: 'bytes' });

        return !!publicKey.current;
      } catch {
        console.log('signRaw does not exist');

        return false;
      }
    }
  };

  const handlePlayStream = useCallback(async () => {
    if (!account?.decodedAddress || !streamId || !socket) {
      return;
    }

    setStreamStatus('loading');

    if (!publicKey.current?.signature) {
      const isPublicKey = await handleGetPublicKey();

      if (!isPublicKey) {
        setStreamStatus('ready-to-play');
        return;
      }
    }

    socket.emit('watch', account?.decodedAddress, {
      streamId,
      signedMsg: publicKey.current?.signature,
      encodedId: account.address,
    });

    peerConnection.current = new RTCPeerConnection(RTC_CONFIG);

    socket.on('offer', (broadcasterId: string, { description, userId }: OfferMsg) => {
      setLocalStream(null);

      peerConnection.current
        ?.setRemoteDescription(description)
        .then(() => peerConnection.current?.createAnswer())
        .then((answer: any) => peerConnection.current?.setLocalDescription(answer))
        .then(() => {
          socket.emit('answer', broadcasterId, {
            userId,
            streamId,
            description: peerConnection.current?.localDescription,
          });
        })
        .catch(() => {
          console.log('error when setLocalDescription');
        });

      peerConnection.current!.onicecandidate = (event: RTCPeerConnectionIceEvent) => {
        if (event.candidate) {
          socket.emit('candidate', broadcasterId, {
            candidate: event.candidate,
            userId,
            streamId,
          });
        }
      };

      peerConnection.current!.ontrack = (event: RTCTrackEvent) => {
        if (event.streams[0]) {
          const audioTracks = event.streams[0].getAudioTracks();
          const videoTracks = event.streams[0].getVideoTracks();

          const str = new MediaStream([...audioTracks, videoTracks[videoTracks.length - 1]]);

          setLocalStream(str);
        } else {
          setLocalStream((prev) => new MediaStream([...(prev ? prev!.getTracks() : []), event.track]));
        }
      };

      peerConnection.current!.onnegotiationneeded = async () => {
        try {
          await peerConnection.current!.setRemoteDescription(description);
        } catch {
          console.log('error when setRemoteDescription');
        }

        peerConnection
          .current!.createAnswer()
          .then((answer) => {
            peerConnection.current!.setLocalDescription(answer);
          })
          .then(() => {
            socket.emit('answer', broadcasterId, {
              watcherId: account?.decodedAddress,
              description: peerConnection.current?.localDescription,
            });
          })
          .catch(() => {
            console.log('error when setLocalDescription');
          });
      };
    });

    socket.on('error', ({ message }: ErrorMsg) => {
      if (message === `Stream with id ${streamId} hasn't started yet`) {
        setStreamStatus('not-started');
      }
      if (message === `You aren't subscribed to this speaker`) {
        setStreamStatus('not-subscribed');
      }
    });

    socket.on('candidate', (_: string, msg: CandidateMsg) => {
      peerConnection.current?.addIceCandidate(new RTCIceCandidate(msg.candidate)).catch((err) => console.error(err));
    });

    socket.on('stopBroadcasting', () => {
      setStreamStatus('ended');
      peerConnection.current?.close();
      peerConnection.current = null;
    });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [account?.address, account?.decodedAddress, socket, streamId]);

  useEffect(
    () => () => {
      socket.emit('stopWatching', account?.decodedAddress, { streamId });
      peerConnection.current?.close();
      peerConnection.current = null;
      socket.off();
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [streamId],
  );

  useEffect(() => {
    if (remoteVideo.current && localStream) {
      setStreamStatus('streaming');
      remoteVideo.current.srcObject = localStream;
      remoteVideo.current
        .play()
        .then((s) => s)
        .catch((err) => console.log(err));
    }
  }, [localStream]);

  const autoWatch = useCallback(async () => {
    if (streamStatus === 'not-started') {
      if (!publicKey.current?.signature) {
        const isPublicKey = await handleGetPublicKey();

        if (!isPublicKey) {
          return;
        }
      }

      if (account?.address && !retryIntervalId.current) {
        retryIntervalId.current = setInterval(() => {
          socket.emit('watch', account?.decodedAddress, {
            streamId,
            signedMsg: publicKey.current?.signature,
            encodedId: account.address,
          });
        }, 2000);
      }
    }

    if (streamStatus !== 'not-started') {
      if (retryIntervalId.current) {
        clearInterval(retryIntervalId.current);
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [streamStatus, account?.address, account?.decodedAddress, streamId, socket]);

  useEffect(() => {
    autoWatch();
  }, [autoWatch]);

  const handlePlayerReady = (player: HTMLVideoElement) => {
    remoteVideo.current = player;
  };

  const handleGoToStreams = () => {
    navigate('/account');
  };

  return (
    <div className={cx(styles.layout)}>
      <Player onReady={handlePlayerReady} mode="watch" />
      {streamStatus === 'loading' && (
        <div className={cx(styles['broadcast-not-available'])}>
          <Loader />
        </div>
      )}
      {streamStatus === 'ready-to-play' && (
        <div className={cx(styles['broadcast-not-available'])}>
          {streamTeasers?.[streamId].imgLink && (
            <>
              <img
                src={streamTeasers?.[streamId].imgLink}
                alt="stream background"
                className={cx(styles['stream-background'])}
              />
              <div className={cx(styles['backdrop-filter'])} />
            </>
          )}
          <Button
            variant="primary"
            label="Play Stream"
            className={cx(styles['play-stream-button'])}
            onClick={handlePlayStream}
          />
        </div>
      )}
      {streamStatus === 'not-subscribed' && (
        <div className={cx(styles['broadcast-not-available'])}>
          <h3>Broadcast not available</h3>
          <span>In order to watch the broadcast, you need to subscribe to this streamer</span>
        </div>
      )}
      {streamStatus === 'not-started' && (
        <div className={cx(styles['broadcast-not-available'])}>
          <h3>Stream in not started yet</h3>
        </div>
      )}
      {streamStatus === 'ended' && (
        <div className={cx(styles['broadcast-not-available'])}>
          <h3>Broadcast has been ended</h3>
          <Button variant="primary" label="Go to streams" onClick={handleGoToStreams} />
        </div>
      )}
    </div>
  );
}

export { Watch };
