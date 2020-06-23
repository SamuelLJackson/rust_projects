FROM debian:bullseye-slim
WORKDIR /app
ADD target/release/actix_tutorial .
CMD ["/app/actix_tutorial"]