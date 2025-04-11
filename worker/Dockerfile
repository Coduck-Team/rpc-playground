FROM debian:bullseye

RUN apt-get update && \
    apt-get install -y g++ gcc default-jdk python3 pypy3 && \
    apt-get clean

WORKDIR /app
COPY compile.sh /app/
COPY run.sh /app/
COPY checker.sh /app/
COPY generator.sh /app/
RUN chmod +x /app/*.sh

