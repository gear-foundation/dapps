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

COPY ./frontend/apps/non-fungible-token ./apps/non-fungible-token
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG VITE_NODE_ADDRESS  \
    VITE_IPFS_ADDRESS  \
    VITE_CONTRACT_ADDRESS  \
    VITE_IPFS_GATEWAY_ADDRESS \
    VITE_SENTRY_DSN
ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_IPFS_ADDRESS=${VITE_IPFS_ADDRESS} \
    VITE_CONTRACT_ADDRESS=${VITE_CONTRACT_ADDRESS} \
    VITE_IPFS_GATEWAY_ADDRESS=${VITE_IPFS_GATEWAY_ADDRESS} \
    VITE_SENTRY_DSN=${VITE_SENTRY_DSN} \
    DISABLE_ESLINT_PLUGIN=true


WORKDIR /frontend/apps/non-fungible-token

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/non-fungible-token/build"]
