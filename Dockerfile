# Stage 1: Build the Rust agent-core binary
FROM rust:1.73-slim-bullseye as agent-server-builder

WORKDIR /build-context

# Install only the minimal required build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    musl-tools \
    && rm -rf /var/core/apt/lists/*

# Build for musl to ensure static linking
RUN rustup target add aarch64-unknown-linux-musl

# Copy only necessary files for building
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY assets ./assets

# Build with musl target for static linking
RUN cargo build --release --target aarch64-unknown-linux-musl && \
    strip /build-context/target/aarch64-unknown-linux-musl/release/agent-core

# Stage 2: Build Bun dependencies
FROM oven/bun:alpine as node-builder

# Install system dependencies and node-gyp
RUN apk add --no-cache \
        ca-certificates \
        curl \
        unzip \
        git \
        python3 \
        py3-pip \
        build-base \
        pkgconf \
        cmake \
        nodejs \
        npm \
        bash \
        chromium \
        nss \
        freetype \
        freetype-dev \
        harfbuzz \
        ca-certificates \
        ttf-freefont \
        font-noto-emoji \
        nodejs \
        wqy-zenhei \
      && rm -rf /var/cache/* \
      && mkdir /var/cache/apk \
      && npm install -g node-gyp

WORKDIR /app
# dep files
COPY packages packages
COPY package.json package-lock.json ./
## Install deps
RUN bun install && bun --filter='./packages/genaiscript-rust-shim' run buildShim


FROM node:20-bookworm as app

WORKDIR /app

# Install playwright
ENV PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH=/usr/lib/chromium/chromium \
  PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1
RUN npx -y playwright@1.49.1 install --with-deps chromium && npm install -g bun


COPY --from=node-builder /app/node_modules ./node_modules
COPY --from=node-builder /app/package.json .
COPY --from=agent-server-builder /build-context/target/aarch64-unknown-linux-musl/release/agent-server ./agent-server

# Ensure the binary is executable
RUN chmod +x ./agent-core

# copy agent source files
COPY packages/genaiscript-rust-shim/dist ./dist
COPY genaisrc ./genaisrc

# Expose the required port
EXPOSE 3006

# Set the entrypoint to the Rust binary
ENTRYPOINT ["./agent-server"]
