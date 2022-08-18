FROM debian:sid

COPY os-release /etc/os-release

COPY bootstrap-action.sh /bootstrap-action.sh
RUN /bootstrap-action.sh

COPY find-and-rename-pkg.sh /find-and-rename-pkg.sh
