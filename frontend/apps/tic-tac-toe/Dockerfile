FROM node:18-alpine
MAINTAINER gear

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases

COPY ./frontend/apps/tic-tac-toe ./apps/tic-tac-toe
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG VITE_GAME_ADDRESS \
    VITE_NODE_ADDRESS
ENV VITE_GAME_ADDRESS=${VITE_GAME_ADDRESS} \
    VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    DISABLE_ESLINT_PLUGIN=true
    
WORKDIR /frontend/apps/tic-tac-toe

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/tic-tac-toe/build"]
