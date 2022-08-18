FROM ubuntu:20.04

COPY bootstrap-action.sh /bootstrap-action.sh
RUN /bootstrap-action.sh

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --default-toolchain stable -y
RUN bash -c "source $HOME/.cargo/env && cargo install --root /bin cargo-deb"

COPY find-and-rename-pkg.sh /find-and-rename-pkg.sh
