FROM debian:bullseye-slim
WORKDIR /app
COPY target/release/rs-hello-world /app/rs-hello-world
EXPOSE 8080
CMD ["./rs-hello-world"]
