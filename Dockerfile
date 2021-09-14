FROM rust:1.54.0

WORKDIR /usr/src/app
COPY . .

RUN cargo install

CMD ["kubers-api"]