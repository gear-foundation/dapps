FROM node:18-alpine

COPY package.json .
COPY tsconfig.json .

RUN yarn install

CMD ["yarn", "start"]
