# select build image
FROM rust:1.42 as build

# create a new empty shell project
RUN USER=root cargo new --bin todo
WORKDIR /todo

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs
RUN rm -rf .env

# copy your source tree
COPY ./src ./src
COPY ./.env ./.env

# build for release
RUN rm ./target/release/deps/todo*
RUN cargo build --release

# our final base
FROM rust:1.42

# copy the build artifact from the build stage
COPY --from=build /todo/target/release/todo .

# set the startup command to run your binary
CMD ["./todo"]