FROM node:16-alpine

ARG REACT_APP_NODE_ADDRESS  \
    REACT_APP_BATTLE_ADDRESS 
ENV REACT_APP_NODE_ADDRESS=${REACT_APP_NODE_ADDRESS} \
    REACT_APP_BATTLE_ADDRESS=${REACT_APP_BATTLE_ADDRESS}
    
COPY package.json yarn.lock tsconfig.json .

WORKDIR tamagotchi

RUN yarn install
CMD ["yarn", "start"]
