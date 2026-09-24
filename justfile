# cargo install justfile
all:
    just --list

init:
    cargo run --release -- init ./test

dev:
    cargo run --release -- dev ./test

build:
    cargo run --release -- build ./test

install:
    cargon install --path .
