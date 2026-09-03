# Moix Client

Create a compressed (`brotli`) version of index.html with svg, javascript, css and embedded templates.

```bash
# moix OPTION PATH
moix init ./test
moix build ./test
moix dev ./test
# http://localhost:8080
```

## Installation

### Production

```bash
cargo install moix-cli
```

### Testing

```bash
cargo install --git https://github.com/moixcc/moix-cli
```

### Development

```bash
cargo install --git https://github.com/moixcc/moix-cli --branch dev
```
