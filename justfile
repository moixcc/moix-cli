# cargo install --locked just
all:
    just --list

tools:
    cargo install --locked \
        just just-lsp cargo-bloat

bloat:
    cargo bloat --release --crates

install: format
    cargo install --path .

format:
    cargo fmt
