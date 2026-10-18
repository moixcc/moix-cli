# cargo install just
all:
    just --list

tools:
    cargo install just cargo-bloat

bloat:
    cargo bloat --release

install: test
    cargo install --path .

test: format
    cargo test --release

format:
    cargo fmt
