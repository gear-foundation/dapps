FROM node:18-alpine

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases

COPY ./frontend/apps/game-of-chance ./apps/game-of-chance
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG REACT_APP_NODE_ADDRESS \
    REACT_APP_CONTRACT_ADDRESS
ENV REACT_APP_NODE_ADDRESS=${REACT_APP_NODE_ADDRESS} \
    REACT_APP_CONTRACT_ADDRESS=${REACT_APP_CONTRACT_ADDRESS} \
    DISABLE_ESLINT_PLUGIN=true

WORKDIR /frontend/apps/game-of-chance

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/game-of-chance/build"]
