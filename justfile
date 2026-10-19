# cargo install just
all:
    just --list

tools:
    cargo install just cargo-bloat

bloat:
    cargo bloat --release --crates

install: format
    cargo install --path .

format:
    cargo fmt
