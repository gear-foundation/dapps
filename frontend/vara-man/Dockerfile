FROM node:18-alpine

WORKDIR /usr/src

COPY . /usr/src

RUN apk update

RUN apk add xsel

ARG VITE_NODE_ADDRESS \
    VITE_GAME_ADDRESS

ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_GAME_ADDRESS=${VITE_GAME_ADDRESS} \
    DISABLE_ESLINT_PLUGIN=true

RUN yarn install

RUN yarn run build

CMD ["yarn", "run", "dev"]
