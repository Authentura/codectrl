FROM ubuntu:latest

COPY bootstrap-action.sh /bootstrap-action.sh
RUN /bootstrap-action.sh

RUN useradd -d /github/home -g root -G sudo -ms /bin/bash github
USER github

RUN mkdir -p "$HOME"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --default-toolchain stable -y

USER root
RUN bash -c "source /github/home/.cargo/env && cargo install --root /bin cargo-deb"

COPY find-and-rename-pkg.sh /find-and-rename-pkg.sh
