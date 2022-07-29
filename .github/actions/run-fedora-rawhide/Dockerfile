FROM fedora:rawhide

COPY bootstrap-build.sh /tmp/bootstrap-build.sh

RUN /tmp/bootstrap-build.sh
RUN rm /tmp/bootstrap-build.sh
RUN source $HOME/.cargo/env && cargo install cargo-generate-rpm

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT /entrypoint.sh
