# Build stage
FROM node:18-alpine AS build

WORKDIR /opt/fe

COPY . /opt/fe

ARG VITE_NODE_ADDRESS \
    VITE_FT_ADDRESS \
    VITE_GAME_ADDRESS \
    VITE_AUTH_API_ADDRESS \
    VITE_TESTNET_WEBSITE_ADDRESS

ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_FT_ADDRESS=${VITE_FT_ADDRESS} \
    VITE_GAME_ADDRESS=${VITE_GAME_ADDRESS} \
    VITE_AUTH_API_ADDRESS=${VITE_AUTH_API_ADDRESS} \
    VITE_TESTNET_WEBSITE_ADDRESS=${VITE_TESTNET_WEBSITE_ADDRESS}

RUN apk update --no-cache && \
    apk add --no-cache xsel && \
    yarn install && \
    yarn build && \
    rm -rf /var/cache/apk/*

# Run stage
FROM node:18-alpine

WORKDIR /opt/fe

COPY . /opt/fe
COPY --from=build /opt/fe/dist /opt/fe/dist
COPY --from=build /opt/fe/node_modules /opt/fe/node_modules

ARG VITE_NODE_ADDRESS \
    VITE_FT_ADDRESS \
    VITE_GAME_ADDRESS \
    VITE_AUTH_API_ADDRESS \
    VITE_TESTNET_WEBSITE_ADDRESS

ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_FT_ADDRESS=${VITE_FT_ADDRESS} \
    VITE_GAME_ADDRESS=${VITE_GAME_ADDRESS} \
    VITE_AUTH_API_ADDRESS=${VITE_AUTH_API_ADDRESS} \
    VITE_TESTNET_WEBSITE_ADDRESS=${VITE_TESTNET_WEBSITE_ADDRESS}

EXPOSE 3000

CMD ["yarn", "preview"]
