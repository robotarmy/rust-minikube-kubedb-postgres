FROM rust:1.26.0 as build

ENV PROJ=ledger

#shell
RUN USER=root cargo new --bin $PROJ
WORKDIR /$PROJ

COPY ./Cargo.toml ./Cargo.toml


# Builds your dependencies and removes the
# fake source code from the dummy project
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src


RUN cargo build --release
RUN ls -al target/release
# our final base
FROM rust:1.26.0

ENV PROJ=ledger
# copy the build artifact from the build stage
COPY --from=build /$PROJ/target/release/$PROJ .

# set the startup command to run your binary
CMD [$PROJ]

