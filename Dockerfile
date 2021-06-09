FROM rust:1.52.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

RUN mkdir /app
COPY . /app
WORKDIR /app

RUN if [ "$DOMAIN" = "cn" ] ; then compose/cn_build.sh ; fi

RUN cargo install --path .


CMD ["explorer"]