FROM node:18-alpine
MAINTAINER gear

WORKDIR /usr/src

COPY . /usr/src

RUN apk update

RUN apk add xsel

ARG REACT_APP_NODE_ADDRESS
ENV REACT_APP_NODE_ADDRESS=${REACT_APP_NODE_ADDRESS} \
    DISABLE_ESLINT_PLUGIN=true 
    
RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "-s", "/usr/src/build"]
