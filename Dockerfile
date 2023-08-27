FROM rust:1.72.0

RUN apt-get update -yqq && apt-get install -yqq cmake g++

ADD ./ /actix
WORKDIR /actix

RUN cargo clean
RUN make target

EXPOSE 80

CMD ./target/release/cotametais-lme-api-rust