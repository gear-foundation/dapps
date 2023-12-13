FROM node:18-alpine

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases

COPY ./frontend/apps/battleship ./apps/battleship
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG VITE_NODE_ADDRESS \
    VITE_CONTRACT_ADDRESS \
    VITE_SENTRY_DSN \
    VITE_GASLESS_BACKEND_ADDRESS

ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_CONTRACT_ADDRESS=${VITE_CONTRACT_ADDRESS} \
    VITE_SENTRY_DSN=${VITE_SENTRY_DSN} \
    VITE_GASLESS_BACKEND_ADDRESS=${VITE_GASLESS_BACKEND_ADDRESS}

WORKDIR /frontend/apps/battleship

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/battleship/build"]
