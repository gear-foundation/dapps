import { useAccount } from '@gear-js/react-hooks';
import { SignerResult } from '@polkadot/api/types';
import { stringToHex } from '@polkadot/util';
import { RefObject, useEffect, useRef, useState, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';

import { useGetStateQuery } from '@/app/utils';
import { Loader } from '@/components';
import { Button } from '@/ui';
import { cx } from '@/utils';

import { RTC_CONFIG } from '../../config';
import { Player } from '../Player';

import { CandidateMsg, ErrorMsg, OfferMsg, StreamState, WatchProps } from './Watch.interface';
import styles from './Watch.module.scss';

function Watch({ socket, streamId }: WatchProps) {
  const navigate = useNavigate();
  const publicKey: RefObject<SignerResult | null> = useRef(null);
  const retryIntervalId: RefObject<ReturnType<typeof setInterval> | null> = useRef(null);
  const remoteVideo: RefObject<HTMLVideoElement | null> = useRef(null);
  const [localStream, setLocalStream] = useState<MediaStream | null>(null);
  const peerConnection: RefObject<RTCPeerConnection | null> = useRef(null);
  const { account } = useAccount();
  const { streams } = useGetStateQuery();

  const [streamStatus, setStreamStatus] = useState<StreamState>('ready-to-play');

  const handleGetPublicKey = async () => {
    if (account?.address && !publicKey.current) {
      const { address } = account;

      try {
        const { signer } = account;

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
    const activeConnection = peerConnection.current;

    socket.on('offer', async (broadcasterId: string, { description, userId }: OfferMsg) => {
      setLocalStream(null);

      try {
        await activeConnection?.setRemoteDescription(description);
        const answer = await activeConnection?.createAnswer();
        if (answer) {
          await activeConnection.setLocalDescription(answer);
        }

        socket.emit('answer', broadcasterId, {
          userId,
          streamId,
          description: activeConnection?.localDescription,
        });
      } catch {
        console.log('error when setLocalDescription');
      }

      if (!activeConnection) {
        return;
      }

      activeConnection.onicecandidate = (event: RTCPeerConnectionIceEvent) => {
        if (event.candidate) {
          socket.emit('candidate', broadcasterId, {
            candidate: event.candidate,
            userId,
            streamId,
          });
        }
      };

      activeConnection.ontrack = (event: RTCTrackEvent) => {
        if (event.streams[0]) {
          const audioTracks = event.streams[0].getAudioTracks();
          const videoTracks = event.streams[0].getVideoTracks();

          const str = new MediaStream([...audioTracks, videoTracks[videoTracks.length - 1]]);

          setLocalStream(str);
        } else {
          setLocalStream((prev) => new MediaStream([...(prev ? prev.getTracks() : []), event.track]));
        }
      };

      activeConnection.onnegotiationneeded = async () => {
        try {
          await activeConnection.setRemoteDescription(description);
        } catch {
          console.log('error when setRemoteDescription');
        }

        try {
          const answer = await activeConnection.createAnswer();
          await activeConnection.setLocalDescription(answer);
          socket.emit('answer', broadcasterId, {
            watcherId: account?.decodedAddress,
            description: peerConnection.current?.localDescription,
          });
        } catch {
          console.log('error when setLocalDescription');
        }
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
      if (!activeConnection) {
        return;
      }

      void activeConnection.addIceCandidate(msg.candidate).catch((err) => console.error(err));
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
    const video = remoteVideo.current;
    if (video && localStream) {
      setStreamStatus('streaming');
      video.srcObject = localStream;

      void video.play().catch((err) => console.log(err));
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

      if (account?.address && account?.decodedAddress && !retryIntervalId.current) {
        const { address } = account;
        retryIntervalId.current = setInterval(() => {
          socket.emit('watch', account.decodedAddress, {
            streamId,
            signedMsg: publicKey.current?.signature,
            encodedId: address,
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
    void autoWatch();
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
          {streams?.[streamId].img_link && (
            <>
              <img
                src={streams?.[streamId].img_link}
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
