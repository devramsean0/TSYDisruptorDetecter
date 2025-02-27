FROM rust:latest AS cli

LABEL org.opencontainers.image.source="https://github.com/devramsean0/TSYDisruptorDetecter"

WORKDIR /usr/src/cli

COPY cli/Cargo.toml .
COPY cli/Cargo.lock .
COPY cli/src src/

RUN cargo build --release

FROM node:latest AS bot-base

RUN apt install g++ make python3

RUN npm install -g node-gyp

WORKDIR /usr/src/bot

COPY bot/package.json .
COPY bot/yarn.lock .
COPY bot/.yarnrc.yml .
COPY bot/.yarn .yarn/

COPY bot/src src/
COPY bot/drizzle drizzle/
COPY bot/drizzle.config.ts drizzle.config.ts

COPY bot/tsup.config.ts tsup.config.ts

ENV NODE_TLS_REJECT_UNAUTHORIZED=0

RUN yarn install --inline-builds
RUN ls
RUN yarn build

FROM bot-base AS bot-runner
WORKDIR /usr/src/bot

COPY --from=bot-base /usr/src/bot/dist dist/
COPY --from=bot-base /usr/src/bot/drizzle drizzle/

# Copy cli
RUN mkdir bin
COPY --from=cli /usr/src/cli/target/release/tsy_disruptions_detector bin/
ENV PATH=/usr/src/bot/bin:$PATH

CMD [ "yarn", "run", "start" ]