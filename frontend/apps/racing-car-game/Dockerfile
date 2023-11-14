FROM node:18-alpine
MAINTAINER gear

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases

COPY ./frontend/apps/racing-car-game ./apps/racing-car-game
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG REACT_APP_CONTRACT_ADDRESS \
    REACT_APP_NODE
ENV REACT_APP_CONTRACT_ADDRESS=${REACT_APP_CONTRACT_ADDRESS} \
    REACT_APP_NODE=${REACT_APP_NODE} \
    DISABLE_ESLINT_PLUGIN=true
    
WORKDIR /frontend/apps/racing-car-game

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/racing-car-game/build"]
