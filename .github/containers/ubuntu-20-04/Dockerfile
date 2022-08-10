FROM ubuntu:20.04

COPY bootstrap-action.sh /bootstrap-action.sh
RUN /bootstrap-action.sh

RUN useradd -d /github/home -g root -G sudo -ms /bin/bash github
USER github

RUN mkdir -p "$HOME"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --default-toolchain stable -y
RUN bash -c "source $HOME/.cargo/env && cargo install cargo-deb"
RUN bash -c "source $HOME/.cargo/env && rustup default stable"

USER root

COPY find-and-rename-pkg.sh /find-and-rename-pkg.sh

USER github
ENTRYPOINT ["/bin/bash", "-l"]
CMD ["/bin/bash"]
