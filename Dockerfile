FROM rust:1.59

WORKDIR /app

# Diesel
RUN cargo install diesel_cli

# Dependencies
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo 'fn main(){}' > src/main.rs

RUN cargo build --release

# Diesel config
COPY migrations migrations
COPY .env .env

# Static serving
COPY static static

# Executable
RUN rm -rf src
COPY src src

# Start script
RUN echo 'diesel migration run && cargo r --release' > start.sh
RUN chmod 744 start.sh

CMD ["bash", "./start.sh"]
