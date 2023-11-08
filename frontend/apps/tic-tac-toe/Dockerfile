# Build stage
FROM node:18-alpine AS build
MAINTAINER GEAR

WORKDIR /opt/fe

COPY . /opt/fe

ARG VITE_NODE_ADDRESS \
    VITE_GAME_ADDRESS \
    VITE_DEFAULT_NODES_URL \
    VITE_STAGING_NODES_URL

ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_GAME_ADDRESS=${VITE_GAME_ADDRESS} \
    VITE_DEFAULT_NODES_URL=${VITE_DEFAULT_NODES_URL} \
    VITE_STAGING_NODES_URL=${VITE_STAGING_NODES_URL}

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
    VITE_GAME_ADDRESS \
    VITE_DEFAULT_NODES_URL \
    VITE_STAGING_NODES_URL

ENV VITE_NODE_ADDRESS=${VITE_NODE_ADDRESS} \
    VITE_GAME_ADDRESS=${VITE_GAME_ADDRESS} \
    VITE_DEFAULT_NODES_URL=${VITE_DEFAULT_NODES_URL} \
    VITE_STAGING_NODES_URL=${VITE_STAGING_NODES_URL}

EXPOSE 3000

CMD ["yarn", "preview"]
