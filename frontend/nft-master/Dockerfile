FROM node:18-alpine

WORKDIR /usr/src

COPY . /usr/src

RUN apk update -y

RUN apk add xsel -y

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "/usr/src/build"]
