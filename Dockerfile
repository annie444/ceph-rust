FROM docker.io/library/buildpack-deps:bookworm

ENV RUSTUP_HOME="/usr/local/rustup" \
    CARGO_HOME="/usr/local/cargo" \
    PATH="/usr/local/cargo/bin:$PATH" \
    RUST_VERSION="stable"

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
    amd64) rustArch='x86_64-unknown-linux-gnu';; \
    armhf) rustArch='armv7-unknown-linux-gnueabihf';; \
    arm64) rustArch='aarch64-unknown-linux-gnu';; \
    i386) rustArch='i686-unknown-linux-gnu';; \
    *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
        --yes --no-modify-path --default-toolchain $RUST_VERSION --default-host $rustArch; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN apt-get update \
 && apt-get update -y \
 && apt-get install -y cephadm \
 && cephadm add-repo --release tentacle \
 && apt-get update -y \
 && apt-get install -y --no-install-recommends \
    uuid-runtime ceph-mgr ceph-mon ceph-osd ceph-mds \
    librados-dev libradosstriper-dev

# update crates.io index
RUN cargo search --limit 1

WORKDIR /ceph-rust

COPY micro-osd.sh setup-micro-osd.sh entrypoint.sh /

CMD ["/entrypoint.sh"]
