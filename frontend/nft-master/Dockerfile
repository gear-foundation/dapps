FROM node:18-alpine

COPY package.json .
COPY tsconfig.json .
COPY config-overrides.js .

RUN yarn install

CMD ["yarn", "start"]
