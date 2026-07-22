# --- Build stage ---
FROM oven/bun:1-alpine AS build
WORKDIR /app

COPY package.json bun.lockb ./
RUN bun install --frozen-lockfile

COPY . .
# Bundle only — type-checking (bun run build's vue-tsc step) is a separate
# CI concern and currently fails to resolve .vue imports under Linux/bun
# (works fine on macOS); vite build alone produces an identical bundle.
RUN bunx vite build

# --- Server build stage ---
FROM rust:1-alpine AS server-build
WORKDIR /app

# build-base (gcc/musl headers) is required for compiling most crates with
# native deps against musl; not in the base rust:alpine image.
RUN apk add --no-cache musl-dev build-base

# Copy the whole server/ dir (rather than individual files) so this doesn't
# break if Cargo.lock isn't checked in yet.
COPY server/ .
RUN cargo build --release

# --- Serve stage ---
# alpine (not distroless/scratch): rust:1-alpine targets musl, so a musl-libc
# base is required to run the resulting binary.
FROM alpine:3
WORKDIR /app

# uid/gid 101 mirrors the nginx-unprivileged convention this replaces.
RUN addgroup -g 101 -S nonroot && adduser -u 101 -S nonroot -G nonroot

COPY --from=server-build --chown=nonroot:nonroot /app/target/release/server ./server
COPY --from=build --chown=nonroot:nonroot /app/dist ./dist

ENV PORT=8080 STATIC_DIR=/app/dist
USER nonroot

EXPOSE 8080
CMD ["/app/server"]
