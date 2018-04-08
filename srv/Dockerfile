FROM ubuntu:latest AS build

RUN apt-get update && apt-get install -y curl build-essential libpq-dev libssl-dev pkg-config && \
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

COPY . /opt/hpas_build
WORKDIR /opt/hpas_build
RUN /root/.cargo/bin/cargo build --release

FROM ubuntu:latest
RUN apt-get update && apt-get install -y libpq-dev libssl-dev
COPY --from=build /opt/hpas_build/target/release/hpas /opt/hpas/bin/hpas
VOLUME /opt/hpas/conf
WORKDIR /opt/hpas
EXPOSE 8080/tcp
ENTRYPOINT ["/opt/hpas/bin/hpas", "--config", "/opt/hpas/conf/config.toml"]