FROM rs-deps:1.74.0-alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/rs-hello-world .
EXPOSE 8080
CMD ["./rs-hello-world"]
