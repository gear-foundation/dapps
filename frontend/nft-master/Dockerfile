FROM node:18-alpine
MAINTAINER GEAR

WORKDIR /usr/src

COPY . /usr/src

RUN apk update

RUN apk add xsel

ARG REACT_APP_NODE_ADDRESS \
    REACT_APP_DEFAULT_NODES_URL \
    REACT_APP_IPFS_GATEWAY_ADDRESS \
    REACT_APP_MASTER_NFT_ADDRESS \
    REACT_APP_AUTH_API_ADDRESS \
    REACT_APP_NFT_STATE_SOCKET \
    REACT_APP_NFT_EXPLORER_URL
ENV REACT_APP_NODE_ADDRESS=${REACT_APP_NODE_ADDRESS} \
    REACT_APP_IPFS_GATEWAY_ADDRESS=${REACT_APP_IPFS_GATEWAY_ADDRESS} \
    REACT_APP_MASTER_NFT_ADDRESS=${REACT_APP_MASTER_NFT_ADDRESS} \
    REACT_APP_AUTH_API_ADDRESS=${REACT_APP_AUTH_API_ADDRESS} \
    REACT_APP_NFT_STATE_SOCKET=${REACT_APP_NFT_STATE_SOCKET} \
    REACT_APP_NFT_EXPLORER_URL=${REACT_APP_NFT_EXPLORER_URL} \
    DISABLE_ESLINT_PLUGIN=true

# Install dependencies based on the preferred package manager
RUN \
    if [ -f yarn.lock ]; then yarn --frozen-lockfile; \
    elif [ -f package-lock.json ]; then npm ci; \
    elif [ -f pnpm-lock.yaml ]; then yarn global add pnpm && pnpm i; \
    else echo "Lockfile not found." && exit 1; \
    fi

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/usr/src/build"]
