FROM fedora:latest

COPY bootstrap-action.sh /bootstrap-action.sh

RUN /bootstrap-action.sh
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --default-toolchain nightly -y
RUN bash -c "source $HOME/.cargo/env && cargo install cargo-generate-rpm"

COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
