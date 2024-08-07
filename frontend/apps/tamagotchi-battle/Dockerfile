FROM node:18-alpine
MAINTAINER gear

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases

COPY ./frontend/apps/tamagotchi-battle ./apps/tamagotchi-battle
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG REACT_APP_DNS_API_URL \
    REACT_APP_DNS_NAME \
    REACT_APP_NODE_ADDRESS \
    REACT_APP_SENTRY_DSN \    
    REACT_APP_GASLESS_BACKEND_ADDRESS \
    REACT_APP_GTM_ID
ENV REACT_APP_NODE_ADDRESS=${REACT_APP_NODE_ADDRESS} \
    REACT_APP_DNS_API_URL=${REACT_APP_DNS_API_URL} \
    REACT_APP_DNS_NAME=${REACT_APP_DNS_NAME} \
    REACT_APP_GASLESS_BACKEND_ADDRESS=${REACT_APP_GASLESS_BACKEND_ADDRESS} \
    REACT_APP_SENTRY_DSN=${REACT_APP_SENTRY_DSN} \
    REACT_APP_GTM_ID=${REACT_APP_GTM_ID} \
    DISABLE_ESLINT_PLUGIN=true

WORKDIR /frontend/apps/tamagotchi-battle

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/tamagotchi-battle/build"]
