FROM rust:1.52.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /sub-explorer/explorer-converter
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/explorer /usr/local/bin/explorer

CMD ["explorer"]