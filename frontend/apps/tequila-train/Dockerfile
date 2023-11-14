FROM node:18-alpine
MAINTAINER gear

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases

COPY ./frontend/apps/tequila-train ./apps/tequila-train
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG REACT_APP_NODE_ADDRESS  \
    REACT_APP_GAME_ADDRESS
ENV REACT_APP_NODE_ADDRESS=${REACT_APP_NODE_ADDRESS} \
    REACT_APP_GAME_ADDRESS=${REACT_APP_GAME_ADDRESS}

WORKDIR /frontend/apps/tequila-train

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/tequila-train/build"]
