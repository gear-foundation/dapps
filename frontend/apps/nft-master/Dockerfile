FROM node:18-alpine
MAINTAINER gear

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases
COPY /frontend/vite.apps.config.ts .
COPY /frontend/tsconfig.json .
COPY /frontend/tsconfig.node.json .

COPY ./frontend/apps/nft-master ./apps/nft-master
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG VITE_NODE_ADDRESS \
    VITE_IPFS_GATEWAY_ADDRESS \
    VITE_DNS_API_URL \
    VITE_DNS_NAME \
    VITE_IPFS_ADDRESS \
    VITE_SENTRY_DSN \
    VITE_NFT_EXPLORER_URL
ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_IPFS_ADDRESS=${VITE_IPFS_ADDRESS} \
    VITE_IPFS_GATEWAY_ADDRESS=${VITE_IPFS_GATEWAY_ADDRESS} \
    VITE_DNS_API_URL=${VITE_DNS_API_URL} \
    VITE_DNS_NAME=${VITE_DNS_NAME} \
    VITE_SENTRY_DSN=${VITE_SENTRY_DSN} \
    VITE_NFT_EXPLORER_URL=${VITE_NFT_EXPLORER_URL} \
    DISABLE_ESLINT_PLUGIN=true

WORKDIR /frontend/apps/nft-master

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/nft-master/build"]
