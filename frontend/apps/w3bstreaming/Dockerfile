FROM node:18-alpine
MAINTAINER gear

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases

COPY ./frontend/apps/w3bstreaming ./apps/w3bstreaming
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG VITE_NODE_ADDRESS \
    VITE_DNS_API_URL \
    VITE_DNS_NAME \
    VITE_IPFS_GATEWAY_ADDRESS \
    VITE_IPFS_ADDRESS \
    VITE_BACKEND_SERVER \
    VITE_SIGNALING_SERVER
ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_DNS_API_URL=${VITE_DNS_API_URL} \
    VITE_DNS_NAME=${VITE_DNS_NAME} \
    VITE_IPFS_GATEWAY_ADDRESS=${VITE_IPFS_GATEWAY_ADDRESS} \
    VITE_IPFS_ADDRESS=${VITE_IPFS_ADDRESS} \
    VITE_SIGNALING_SERVER=${VITE_SIGNALING_SERVER} \
    VITE_BACKEND_SERVER=${VITE_BACKEND_SERVER} \
    DISABLE_ESLINT_PLUGIN=true

WORKDIR /frontend/apps/w3bstreaming

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/w3bstreaming/build"]
