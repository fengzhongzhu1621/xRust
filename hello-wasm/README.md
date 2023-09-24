# 构建WASM二进制文件
```
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
```

# 创建 Docker Image
```
docker buildx build --platform wasi/wasm32 -t hello-wasm .
```

```
docker run --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm32 hello-wasm
``