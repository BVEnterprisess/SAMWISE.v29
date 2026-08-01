.PHONY: build test run clean docker-up docker-down fmt clippy

build:
cargo build --release

test:
cargo test

run:
cargo run

clean:
cargo clean

fmt:
cargo fmt --all

clippy:
cargo clippy --all-targets --all-features -- -D warnings

docker-up:
docker compose up -d --build

docker-down:
docker compose down
