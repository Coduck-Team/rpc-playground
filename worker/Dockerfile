FROM debian:bullseye

RUN apt-get update && \
    apt-get install -y g++ gcc default-jdk python3 pypy3 && \
    apt-get clean

WORKDIR /app
COPY run_worker.sh /app/
RUN chmod +x /app/run_worker.sh

ENTRYPOINT ["/app/run_worker.sh"]
