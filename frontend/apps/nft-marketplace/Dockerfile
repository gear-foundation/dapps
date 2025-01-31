FROM node:18-alpine

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases

COPY ./frontend/apps/nft-marketplace ./apps/nft-marketplace
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG VITE_NODE_ADDRESS  \
    VITE_IPFS_ADDRESS  \
    VITE_NFT_CONTRACT_ADDRESS  \
    VITE_MARKETPLACE_CONTRACT_ADDRESS \
    VITE_IPFS_GATEWAY_ADDRESS \
    VITE_SENTRY_DSN
ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_IPFS_ADDRESS=${VITE_IPFS_ADDRESS} \
    VITE_MARKETPLACE_CONTRACT_ADDRESS=${VITE_MARKETPLACE_CONTRACT_ADDRESS} \
    VITE_NFT_CONTRACT_ADDRESS=${VITE_NFT_CONTRACT_ADDRESS} \
    VITE_IPFS_GATEWAY_ADDRESS=${VITE_IPFS_GATEWAY_ADDRESS} \
    VITE_SENTRY_DSN=${VITE_SENTRY_DSN} \
    DISABLE_ESLINT_PLUGIN=true

WORKDIR /frontend/apps/nft-marketplace

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/nft-marketplace/build"]
