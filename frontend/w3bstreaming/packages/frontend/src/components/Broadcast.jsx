import { useEffect, useRef, useState } from 'react';
import socket from '../utils/socket.js';
import { RTC_CONFIG } from '../config.js';

export default function Braodcast() {
  const [streamId, setStreamId] = useState('');
  const [pk, setPK] = useState('');
  const [stream, setStream] = useState(null);

  const localVideo = useRef(null);
  const conns = {};

  const start = () => {
    if (streamId === '') {
      alert('Set stream id');
      return;
    }
    navigator.mediaDevices
      .getUserMedia({ audio: true, video: true })
      .then((s) => {
        setStream(s);
        return s;
      })
      .then((s) => {
        socket.emit('broadcast', pk, { streamId });

        socket.on('watch', (id, _) => {
          console.log('watch', id, _);
          const pc = new RTCPeerConnection(RTC_CONFIG);
          conns[id] = pc;
          s.getTracks().forEach((t) => pc.addTrack(t, s));
          pc.onicecandidate = (event) => {
            if (event.candidate) {
              socket.emit('candidate', pk, { id, streamId, candidate: event.candidate });
            }
          };
          pc.createOffer()
            .then((sdp) => pc.setLocalDescription(sdp))
            .then(() => socket.emit('offer', pk, { userId: id, streamId, description: pc.localDescription }));
        });

        socket.on('answer', (id, msg) => {
          console.log('answer', id, msg);
          conns[id].setRemoteDescription(msg.description);
        });

        socket.on('candidate', (id, msg) => {
          console.log('candidate', id, msg);
          conns[id].addIceCandidate(new RTCIceCandidate(msg.candidate));
        });

        socket.on('disconnect', () => {
          console.error('DISCONNECTED');
        });

        socket.on('close', () => {
          console.error('CLOSE');
        });
      });
  };

  const stop = () => {
    socket.emit('stopBroadcasting', pk, { streamId });
    stream.getTracks().forEach((t) => t.stop());
    setStream(null);
    localVideo.current.srcObject = null;
  };

  useEffect(() => {
    if (localVideo.current && stream) {
      localVideo.current.srcObject = stream;
    }
  }, [stream]);

  return (
    <div>
      <h1>Broadcast</h1>
      <label htmlFor="">Public Key: </label>
      <input
        placeholder="PK"
        onChange={(event) => {
          setPK(event.target.value);
        }}
      />
      <br />
      <label htmlFor="">Stream ID: </label>
      <input
        placeholder="ID"
        onChange={(event) => {
          setStreamId(event.target.value);
        }}
      />
      <br /> <br />
      <button onClick={start}>Start</button>
      <button onClick={stop}>Stop</button>
      <br />
      <br />
      <div>
        <video ref={localVideo} className="local" autoPlay playsInline />
      </div>
    </div>
  );
}
