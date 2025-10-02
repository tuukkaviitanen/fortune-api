# Database builder stage for initializing and populating the SQLite database
FROM keinos/sqlite3:3.50.4 AS db-builder

WORKDIR /tmp

COPY ./fortunes.csv ./

RUN sqlite3 database.sqlite 'CREATE TABLE fortunes(fortune TEXT);' \
    && sqlite3 database.sqlite '.import fortunes.csv fortunes --csv'

FROM rust:1.90.0 AS builder

WORKDIR /fortune-api

COPY . .

RUN cargo build --release

# Minimal runtime stage using Chainguard's secure glibc image
FROM cgr.dev/chainguard/glibc-dynamic:latest AS runtime

WORKDIR /app

COPY --from=db-builder /tmp/database.sqlite .
COPY --from=builder /fortune-api/target/release/fortune-api .

ENV DATABASE_PATH=./database.sqlite

CMD ["./fortune-api"]
