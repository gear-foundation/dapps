# W3BStreaming
W3BSteaming is an example of decentrilized streaming application based on WebRTC protocol.

The basic implementation includes 3 main parts
- Contract - responsible for storing streams schedule and managing user subscriptions.
- Frontend - the application interface itself.
- Signaling server - responsible for establishing p2p connections between streamer and watchers.


## Contract

***Prerequisites***
- Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

***Build***
```bash
make build_contract
```

## Signaling server and frontend

### Run from source

***Prerequisites***
- NodeJS >= 18

***Environment variables***
Check the `.env.example` file to get list of necessary variables

***Build***
```bash
make init
make build_js
```

***Run***
- Signaling server
```bash
make run_server
```
