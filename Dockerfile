FROM rust:alpine AS build-indexer

RUN apk update
RUN apk add --no-cache build-base openssl-dev sqlite-dev

RUN mkdir /dist
WORKDIR /build

# Build application
COPY meeting-indexer .
RUN RUSTFLAGS=-Ctarget-feature=-crt-static cargo build --release

RUN cp target/release/meeting-indexer /dist

FROM node:lts-alpine as build-frontend

RUN mkdir /dist
WORKDIR /build

# Install node dependencies
COPY frontend/package.json .
COPY frontend/package-lock.json .
RUN npm install

# Build frontend
COPY docker/.env.production .
COPY frontend .
RUN npm run build

RUN cp -r dist /dist

FROM alpine:latest

RUN mkdir /usr/share/meeting-indexer
VOLUME ["/usr/share/meeting-indexer"]

# Install programs
RUN apk update
RUN apk add --no-cache supervisor nginx apk-cron

# Setup config files
COPY docker/supervisord.conf /etc/supervisord.conf
COPY docker/nginx.conf /etc/nginx/nginx.conf
COPY docker/crontab /etc/crontab

# Copy build files
COPY --from=build-indexer /dist/meeting-indexer /usr/bin/meeting-indexer
COPY --from=build-frontend /dist/* /var/www/html

EXPOSE "80"
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
