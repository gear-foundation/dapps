# Reusable Image for Building Gear Ecosystem Contracts

This workspace contains a Dockerfile and docker-compose for building contracts.

## Supported image

**rust:stable**
Image built on Ubuntu with Rust certain version installed.

## Usage

For building purposes, a prepared docker-compose.yml is provided which can be used for running building or testing commands.

### 🏗️ Example of building command

#### Option 1: Run Docker

```
git clone https://github.com/gear-foundation/dapps
docker run -v ./dapps/contracts:/contracts \
           -v ./output:/contracts/target/wasm32-unknown-unknown \
           ghcr.io/dapps/rust:stable \
           cargo build
```

#### Option 2: Run Docker Compose

Edit `docker-compose.yml` with your paths:
```yaml
version: '3.7'
services:
  contracts:
    image: "ghcr.io/dapps/rust:stable"
    volumes:
      - "../dapps/contracts:/contracts" # <- set PATH to contracts folder on hosts where docker-compose wiil be runned
      - "./output:/contracts/target/wasm32-unknown-unknown" # <- set LOCAL_PATH to output directory
    command: ["cargo","build"] # <- command to run inside container
```

```sh
git clone https://github.com/gear-foundation/dapps
docker compose up # <- run docker compose
```