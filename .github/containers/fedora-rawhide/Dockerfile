FROM fedora:rawhide

USER github
ENV HOME="/github/home"

RUN mkdir -p "$HOME"

COPY bootstrap-action.sh /bootstrap-action.sh
RUN /bootstrap-action.sh

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- --default-toolchain stable -y
RUN bash -c "source $HOME/.cargo/env && cargo install cargo-generate-rpm"
RUN bash -c "source $HOME/.cargo/env && rustup default stable"

COPY find-and-rename-pkg.sh /find-and-rename-pkg.sh

ENTRYPOINT ["/bin/bash", "-l"]
CMD ["/bin/bash"]
