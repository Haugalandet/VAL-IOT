FROM rust:1.73.0

WORKDIR /src

COPY . .

RUN cargo build

CMD [ "cargo", "run" ]