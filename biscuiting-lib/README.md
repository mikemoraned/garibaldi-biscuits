# Building

Basic:

    wasm-pack build --scope mike_moran

Optimised:

    wasm-pack build --scope mike_moran --release

With features:

    wasm-pack build --scope mike_moran --release -- --features "console_tracing"

# Testing

    wasm-pack test --node
    cargo test

# Sharing locally

In current dir:

    cd pkg
    npm link

Elsewhere, where we want to use the locally published version:

    npm link @mike_moran/biscuiting-lib

# Publishing

    wasm-pack publish
