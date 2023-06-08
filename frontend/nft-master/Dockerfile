FROM node:18-alpine

WORKDIR /usr/src

COPY . /usr/src

RUN yarn install

RUN yarn build

RUN npm install --global serve

CMD ["serve", "/usr/src/build"]
