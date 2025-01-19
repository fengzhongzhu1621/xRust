# 安装命令
```
cargo install cargo-nextest
```

# 查找所有测试
```
cargo nextest list
cargo nextest list test_random
```

# 找出慢测试、泄露测试，并设置超时时间，超时就自动终止
```
cargo nextest run --slow-timeout 60 -leak-timeout 1024
```

# 测试指定的包
```
cargo nextest run -p xtool
```

# 在 core_utils 包运行指定名称的测试
```
# 测试 tests 文件夹中的指定函数
# cargo nextest run <test-name1> <test-name2>...
# cargo nextest run test_random_string
# cargo nextest run test_random_string
# cargo nextest run -- test_random_string
# cargo nextest run -E 'test(test_random_string_2)'
# cargo nextest run -E 'test(test_random)'
# 精确匹配函数名
# cargo nextest run -E 'test(=test_random_string)'
# 测试库中的 指定函数
# cargo nextest run --lib random::arbitrary::option::tests::test_option
# cargo nextest run random::arbitrary::option::tests::test_option
# cargo nextest run random::arbitrary::option::tests
# cargo nextest run random::arbitrary::option::
# cargo nextest run random::arbitrary:
# cargo nextest run random::
# 测试 tests 的一个文件
# cargo nextest run --test test_random
# 测试所有单测
# cargo nextest run
```

# 运行上次失败的测试
```
cargo nextest run -- --failed
```

# 并发运行
```
cargo nextest run --release -- --jobs 4
cargo nextest --jobs 4
```

# 重试失败的测试用例
```
cargo nextest run --retries 3
```

# 测试 lib 中的所有测试用例
```
cargo nextest run :
cargo nextest run --lib
```

# 运行项目中的所有测试
```
cargo nextest run
cargo nextest run --tests
```

# cargo-deny 检查你的Rust项目中的许可证冲突、禁止使用的库、版本不一致以及安全漏洞等问题。
## 安装 cargo-deny
```
cargo install --locked cargo-deny
cargo deny init
```

## cargo deny check
```
cargo deny check license
cargo deny check bans
cargo deny check advisories
cargo deny check sources
```