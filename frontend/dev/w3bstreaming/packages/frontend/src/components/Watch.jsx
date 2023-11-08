import { useEffect, useRef, useState } from 'react';
import socket from '../utils/socket.js';
import { RTC_CONFIG } from '../config.js';

export default function Watch() {
  const [streamId, setStreamId] = useState('');
  const [pk, setPk] = useState('');
  const [sig, setSig] = useState('');
  const [stream, setStream] = useState(null);
  const remoteVideo = useRef(null);
  let pc = null;

  const startWatching = () => {
    socket.emit('watch', pk, { streamId, signedMsg: sig });

    socket.on('offer', (id, msg) => {
      console.log('offer', id, msg);
      pc = new RTCPeerConnection(RTC_CONFIG);
      pc.setRemoteDescription(msg.description)
        .then(() => pc.createAnswer())
        .then((sdp) => pc.setLocalDescription(sdp))
        .then(() => {
          socket.emit('answer', pk, { broadcasterId: id, description: pc.localDescription });
        });
      pc.ontrack = (event) => {
        setStream(event.streams[0]);
      };
      pc.onicecandidate = (event) => {
        if (event.candidate) {
          socket.emit('candidate', pk, { id, candidate: event.candidate });
        }
      };
    });

    socket.on('candidate', (_, msg) => {
      console.log('candidate', _, msg);
      pc.addIceCandidate(new RTCIceCandidate(msg.candidate)).catch((e) => console.error(e));
    });
  };

  useEffect(() => {
    if (remoteVideo.current && stream) {
      remoteVideo.current.srcObject = stream;
    }
  }, [stream]);

  return (
    <div>
      <h1>Watch</h1>
      <br />
      <label>Public Key: </label>
      <input type="text" placeholder="PK" onChange={({ target: { value } }) => setPk(value)} />
      <br />
      <label>Stream ID: </label>
      <input type="text" placeholder="Stream ID" onChange={({ target: { value } }) => setStreamId(value)} />
      <br />
      <label>Signature: </label>
      <input type="text" placeholder="Signed message" onChange={({ target: { value } }) => setSig(value)} />
      <br /> <br />
      <button onClick={startWatching}>Watch</button>
      <br />
      {stream && (
        <div>
          <video ref={remoteVideo} className="remote" autoPlay playsInline />
        </div>
      )}
    </div>
  );
}
