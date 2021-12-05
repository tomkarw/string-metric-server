# credit https://windsoilder.github.io/writing_dockerfile_in_rust_project.html
FROM rust:latest as builder

WORKDIR /app

# create a new empty project
RUN cargo init

COPY ./.cargo .cargo
COPY ./vendor vendor
COPY Cargo.toml Cargo.lock ./
# build dependencies, when the source code changes, this build can be cached, no need to compile dependency again.
RUN cargo build
# remove the dummy build.
RUN cargo clean -p string-metric-server

COPY ./src src

RUN cargo install --path .

# second stage.
FROM gcr.io/distroless/cc-debian11
COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin/
CMD ["string-metric-server", "80"]
