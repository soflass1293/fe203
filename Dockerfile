FROM rust:1.69.0-alpine3.17
RUN apk add git
RUN addgroup -S app && adduser -S app -G app
USER app
WORKDIR /home/app
COPY Cargo.* ./
RUN cargo install --path . --verbose
COPY . .
EXPOSE 8000
CMD ["app"]