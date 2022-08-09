FROM fedora:latest

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --default-toolchain nightly -y
RUN bash -c "source $HOME/.cargo/env && cargo install cargo-generate-rpm"

COPY find-and-rename-pkg.sh /find-and-rename-pkg.sh
COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
