FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/rs-hello-world .
EXPOSE 8080
CMD ["./rs-hello-world"]
