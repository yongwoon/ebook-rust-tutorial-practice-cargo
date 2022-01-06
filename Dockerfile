FROM rust:1.56.1

WORKDIR /app

RUN apt-get update; \
    apt-get install -y --no-install-recommends \
        vim git tree wget 

EXPOSE 8080
