# rust version 
FROM rust:1.72

# set the working dir
WORKDIR /backend

COPY ./backend .

RUN cargo build --release

CMD ["target/release/webapp"]

EXPOSE 8080
