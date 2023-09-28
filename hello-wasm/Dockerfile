# syntax=docker/dockerfile:1
FROM scratch
COPY ./target/wasm32-wasi/debug/hello-wasm.wasm /hello.wasm
ENTRYPOINT [ "hello.wasm" ]
