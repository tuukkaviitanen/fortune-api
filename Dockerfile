# Database builder stage for initializing and populating the SQLite database
FROM keinos/sqlite3:3.50.4 AS db-builder

WORKDIR /tmp

COPY ./fortunes.csv ./

RUN sqlite3 database.sqlite 'CREATE TABLE fortunes(fortune TEXT NOT NULL, id INTEGER PRIMARY KEY);' \
    && sqlite3 database.sqlite '.separator "|"' \
    && sqlite3 database.sqlite '.import fortunes.csv fortunes'

FROM rust:1.90.0 AS builder

WORKDIR /app

COPY . .

RUN cargo build --locked --release

# Minimal runtime stage using Chainguard's secure glibc image
FROM cgr.dev/chainguard/glibc-dynamic:latest AS runtime

WORKDIR /app

COPY --from=db-builder /tmp/database.sqlite .
COPY --from=builder /app/target/release/fortune-api .

ENV DATABASE_URL=sqlite://./database.sqlite
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

CMD ["./fortune-api"]
