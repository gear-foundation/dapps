FROM node:18-alpine

WORKDIR /frontend

COPY /frontend/package.json .
COPY /frontend/yarn.lock .
COPY /frontend/.yarnrc.yml .
COPY /frontend/.yarn/releases .yarn/releases
COPY /frontend/vite.apps.config.ts .
COPY /frontend/tsconfig.json .
COPY /frontend/tsconfig.node.json .

COPY ./frontend/apps/battleship-zk ./apps/battleship-zk
COPY ./frontend/packages ./packages

RUN apk update

RUN apk add xsel

ARG VITE_NODE_ADDRESS \
    VITE_DNS_API_URL \
    VITE_DNS_NAME \
    VITE_SENTRY_DSN \
    VITE_GASLESS_BACKEND_ADDRESS \
    VITE_ZK_PROOF_BACKEND_ADDRESS \
    VITE_GTM_ID \
    VITE_VOUCHER_LIMIT

ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_DNS_API_URL=${VITE_DNS_API_URL} \
    VITE_DNS_NAME=${VITE_DNS_NAME} \
    VITE_SENTRY_DSN=${VITE_SENTRY_DSN} \
    VITE_GASLESS_BACKEND_ADDRESS=${VITE_GASLESS_BACKEND_ADDRESS} \
    VITE_ZK_PROOF_BACKEND_ADDRESS=${VITE_ZK_PROOF_BACKEND_ADDRESS} \
    VITE_GTM_ID=${VITE_GTM_ID} \
    VITE_VOUCHER_LIMIT=${VITE_VOUCHER_LIMIT}

WORKDIR /frontend/apps/battleship-zk

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/frontend/apps/battleship-zk/build"]
