FROM rust:1.76

WORKDIR /usr/src/file

COPY  . .
RUN cargo install --path .

CMD ["file"] 