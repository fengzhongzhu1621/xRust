!/bin/sh

# 查找所有测试
# cargo nextest list

# 找出慢测试、泄露测试，并设置超时时间，超时就自动终止
# cargo nextest run --slow-timeout 60 -leak-timeout 1024

# 运行指定名称的测试
# cargo nextest run test_name
# cargo nextest run core_utils::test_zip test_zip_file

# 运行指定模块的测试
# cargo nextest run module::

# 运行上次失败的测试
# cargo nextest run -- --failed

# 并发运行
# cargo nextest run --release -- --jobs 4
# cargo nextest --jobs 4

# 重试失败的测试用例
# cargo nextest run --retries 3

# 运行项目中的所有测试
cargo nextest run
