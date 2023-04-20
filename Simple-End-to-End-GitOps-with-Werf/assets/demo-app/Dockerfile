ARG my_appname=sssbnotifymeserver

FROM rust:slim-bullseye as build
ARG my_appname
ENV my_appname=$my_appname

# create a new empty shell project
RUN USER=root cargo new --bin ${my_appname}
WORKDIR /${my_appname}

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/${my_appname}*
RUN cargo build --release

# our final base
FROM debian:bullseye-slim
ARG my_appname
ENV my_appname=$my_appname

# copy the build artifact from the build stage
COPY --from=build /${my_appname}/target/release/${my_appname} /usr/bin/${my_appname}
EXPOSE 3000

# set the startup command to run your binary
CMD ${my_appname}