FROM node:18-alpine
MAINTAINER gear

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases
COPY /frontend/vite.apps.config.ts .
COPY /frontend/tsconfig.json .
COPY /frontend/tsconfig.node.json .

COPY ./frontend/apps/galactic-express ./apps/galactic-express
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG VITE_NODE_ADDRESS \
    VITE_DNS_API_URL \
    VITE_DNS_NAME  \
    VITE_SENTRY_DSN \
    VITE_GTM_ID
ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_DNS_API_URL=${VITE_DNS_API_URL} \
    VITE_DNS_NAME=${VITE_DNS_NAME} \    
    VITE_SENTRY_DSN=${VITE_SENTRY_DSN} \
    VITE_GTM_ID=${VITE_GTM_ID} \
    DISABLE_ESLINT_PLUGIN=true

WORKDIR /frontend/apps/galactic-express

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/galactic-express/build"]
