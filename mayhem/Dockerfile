# ARGS
ARG project=rust-motd

# Setup Fuzz-Rust Builder
FROM ghcr.io/nottirb/fuzz-rust:latest as builder

ADD . /${project}
WORKDIR /${project}

# Inject a lib.rs file and build the fuzz targets
RUN mv ./mayhem/inject/lib.rs src/lib.rs \
    && cd ./fuzz && cargo +nightly hfuzz build

# Setup Fuzz-Env
FROM ubuntu:20.04
#ghcr.io/nottirb/fuzz-env:latest
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/components_constraints /
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/full_print /
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/parse_config /
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/width /
