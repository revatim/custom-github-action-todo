# This line is pulling a Docker image that has Rust version 1.73 and Alpine Linux installed. 
# The 'AS builder' part is naming this stage of the build 'builder'.
FROM rust:1.73-alpine AS builder
# 'apk add --no-cache musl-dev' is installing the musl-dev package, which is a standard C library
RUN apk add --no-cache musl-dev
#  Change working directory
WORKDIR /usr/src/count-pending-todo-action
# Copy the current directory to the working directory
COPY --link . .
RUN cargo install --path .


# Empty image
FROM scratch
COPY --from=builder /usr/local/cargo/bin/count-pending-todo /usr/local/bin/count-pending-todo
ENTRYPOINT ["/usr/local/bin/count-pending-todo-action"]