FROM node:18-alpine

WORKDIR /usr/src

COPY . /usr/src

RUN apk update

RUN apk add make curl

RUN apk add --virtual build-dependencies build-base gcc musl-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup default nightly-2023-04-25 && rustup target add wasm32-unknown-unknown

ARG WS_ADDRESS \
    REACT_APP_SIGNALING_SERVER \
    PATH_TO_META \
    PROGRAM_ID
ENV WS_ADDRESS=${WS_ADDRESS} \
    REACT_APP_SIGNALING_SERVER=${REACT_APP_SIGNALING_SERVER} \
    PATH_TO_META=${PATH_TO_META} \
    PROGRAM_ID=${PROGRAM_ID}

RUN make init

RUN make build_js

CMD ["make", "-j", "2", "run_server","run_fe"]
