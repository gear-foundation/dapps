FROM node:18-alpine
MAINTAINER gear

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases

COPY ./frontend/apps/varatube ./apps/varatube
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG REACT_APP_NODE_ADDRESS  \
    REACT_APP_IPFS_GATEWAY_ADDRESS \
    REACT_APP_CONTRACT_ADDRESS \
    REACT_APP_FT_CONTRACT_ADDRESS
ENV REACT_APP_NODE_ADDRESS=${REACT_APP_NODE_ADDRESS} \
    REACT_APP_IPFS_GATEWAY_ADDRESS=${REACT_APP_IPFS_GATEWAY_ADDRESS} \
    REACT_APP_CONTRACT_ADDRESS=${REACT_APP_CONTRACT_ADDRESS} \
    REACT_APP_FT_CONTRACT_ADDRESS=${REACT_APP_FT_CONTRACT_ADDRESS}

WORKDIR /frontend/apps/varatube

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/varatube/build"]
