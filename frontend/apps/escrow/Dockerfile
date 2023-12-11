FROM node:18-alpine

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases

COPY ./frontend/apps/escrow ./apps/escrow
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG REACT_APP_NODE_ADDRESS \
    REACT_APP_CODE_ADDRESS \
    REACT_APP_SENTRY_DSN
ENV REACT_APP_NODE_ADDRESS=${REACT_APP_NODE_ADDRESS} \
    REACT_APP_CODE_ADDRESS=${REACT_APP_CODE_ADDRESS} \
    REACT_APP_SENTRY_DSN=${REACT_APP_SENTRY_DSN}

WORKDIR /frontend/apps/escrow

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/escrow/build"]
