FROM openjdk:8

MAINTAINER wjun
MAINTAINER wjunjobs@outlook.com

ARG version=3.3.4

RUN apt update \
    && apt install -y openssh-server \
    && mkdir -p ~/.ssh \
    && ssh-keygen -b 2048 -t rsa -f ~/.ssh/id_rsa -q -N "" \
    && cat ~/.ssh/id_rsa.pub >> ~/.ssh/authorized_keys

WORKDIR /opt

COPY hadoop-${version}.tar.gz hadoop-${version}.tar.gz
COPY run-server.sh /run-server.sh
COPY start-namenode.sh /start-namenode.sh
COPY init-server.sh /init-server.sh

RUN tar -zxf hadoop-${version}.tar.gz \
    && mv hadoop-${version} hadoop

CMD ["sh","/run-server.sh"]

