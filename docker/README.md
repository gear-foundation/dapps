# Reusable Image for building Gear Ecosystem Contracts

This workspace contains dockefile and docker-compose for building contracts.

## Supported image

**rust:stable**<br>
Image build on ubuntu with installed rust stable version.

## Usage

For building purposes prepared `docker-compose` which could be used for running building or testing commands.

### 🏗️ Example of building command

Edit `docker-compose.yml` with your paths:
```yaml
version: '3.7'
services:
  contracts:
    image: "ghcr.io/dapps/rust:stable"
    volumes:
      - "/local_path_to_contracts_folder:/contracts" # <- set LOCAL_PATH to contracts folder on hosts where docker-compose wiil be runned
      - "/output:/contracts/target/wasm32-unknown-unknown" # <- set LOCAL_PATH to output directory
    command: ["cargo","build"] # <- command to run inside container
```

```sh
docker-compose up # <- run docker-compose
```