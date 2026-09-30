# cargo install justfile
all:
    just --list

run: format
    cargo run

moix OPTION PATH: format
    cargo run -- {{ OPTION }} {{ PATH }}

format:
    cargo fmt

install: format
    cargo install --path .

init:
    @just moix init ./test

build:
    @just moix build ./test

dev:
    @just moix dev ./test

deploy:
    @just moix deploy ./test

cdn:
    cargo run -- cdn ./test data.json

b64:
    @just moix b64 ./test/cdn/data.json
