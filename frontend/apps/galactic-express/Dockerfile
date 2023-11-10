FROM node:18-alpine
MAINTAINER gear

WORKDIR /src

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .

COPY ./frontend/apps/galactic-express ./frontend/apps/galactic-express
COPY ./frontend/packages ./frontend/packages

RUN apk update

RUN apk add xsel

ARG REACT_APP_NODE_ADDRESS
ENV REACT_APP_NODE_ADDRESS=${REACT_APP_NODE_ADDRESS} \
    DISABLE_ESLINT_PLUGIN=true 
    
RUN yarn install

RUN yarn build:galactic-express

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/galactic-expres/build"]
