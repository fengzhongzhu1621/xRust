# xRust

Rust Study Notes


## 升级 rust

```sh
rustup target remove wasm32-wasi
rustup update stable
rustup --version
# 查看rust版本
rustc --version

cargo update
```

# rust支持的Target列表
```sh
rustup target list
```

# 配置镜像源
```
rm ~/.cargo/.package-cache
```

# cargo
## 升级软件
```sh
cargo update
```

## 初始化项目
```
cargo new --bin hello
cargo new --lib hello
```
