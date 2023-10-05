FROM node:18-alpine
MAINTAINER GEAR

WORKDIR /usr/src

COPY . /usr/src

ARG REACT_APP_NODE_ADDRESS  \
    REACT_APP_GAME_ADDRESS
ENV REACT_APP_NODE_ADDRESS=${REACT_APP_NODE_ADDRESS} \
    REACT_APP_GAME_ADDRESS=${REACT_APP_GAME_ADDRESS}

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/usr/src/build"]
