# syntax=docker/dockerfile:1

#############################################################################
# Builder container                                                         #
#############################################################################
FROM --platform=$BUILDPLATFORM ghcr.io/bitskico/rust-sdk AS builder

# Expose build env variables
ARG TARGETARCH

# Expose GitHub Actions cache args
ARG ACTIONS_CACHE_URL
ARG ACTIONS_RUNNER_DEBUG
ARG ACTIONS_RUNTIME_TOKEN
ARG GITHUB_SHA
ARG SCCACHE_GHA_CACHE_MODE

# Build server
ENV CARGO_INSTALL_ROOT=/usr/local
RUN --mount=target=. \
    --mount=type=cache,target=/var/cache/cargo/target,sharing=private \
    cargo install --locked --path bitski-apns-cli

#############################################################################
# Release container                                                         #
#############################################################################
FROM registry.access.redhat.com/ubi8/ubi-minimal as release

COPY --from=builder /usr/local/bin/* /usr/local/bin/

CMD ["/usr/local/bin/bitski-apns"]
