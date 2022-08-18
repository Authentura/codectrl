FROM centos:7

COPY dnf.conf /etc/dnf/dnf.conf

COPY bootstrap-action.sh /bootstrap-action.sh
RUN /bootstrap-action.sh

COPY find-and-rename-pkg.sh /find-and-rename-pkg.sh
