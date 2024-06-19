FROM rust:1.73.0-alpine as backend-builder

RUN apk update && apk upgrade
RUN apk add --no-cache musl-dev

WORKDIR /app
COPY . .

RUN cargo build --package web --release --locked

# ---

FROM alpine:latest

ENV RUST_LOG=info
ENV APPLICATION_MODE=production
ENV APPLICATION_HOST=0.0.0.0
ENV APPLICATION_PORT=8080

COPY --from=backend-builder /app/target/release/web /backend/web

EXPOSE 8080

CMD ["/backend/web"]
