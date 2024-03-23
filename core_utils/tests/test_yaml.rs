use schemars::schema::RootSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct TestConf {
    debug: bool,
}

impl Default for TestConf {
    fn default() -> Self {
        Self { debug: false }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GlobalConf {
    test: TestConf,
}

impl Default for GlobalConf {
    fn default() -> Self {
        Self { test: TestConf::default() }
    }
}

/// 读取yaml配置文件
fn load_yaml_config<T>(path: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    // 将yam解析为json对象通过
    match serde_yaml::from_str::<RootSchema>(
        &std::fs::read_to_string(path)
            .expect(&format!("failure read config file {}", path)),
    ) {
        Ok(root_schema) => {
            // 将json对象转换指定的model
            let data = serde_json::to_string_pretty(&root_schema)
                .expect("failure to parse RootSchema");
            let config = serde_json::from_str::<T>(&*data)
                .expect(&format!("failure to read yaml file"));

            Some(config)
        }
        Err(_err) => None,
    }
}

/// 读取并加载配置文件
fn fetch_conf(path: Option<PathBuf>) -> GlobalConf {
    match path {
        Some(path) => {
            // 从命令行加载配置
            load_yaml_config::<GlobalConf>(path.to_str().unwrap()).unwrap()
        }
        None => {
            // 配置文件不存在，则使用默认配置
            GlobalConf::default()
        }
    }
}

#[test]
fn test_read_yaml_conf() {
    let config_path = env::current_dir().unwrap().join("tests/data.yaml");
    let global_conf = fetch_conf(Some(config_path));
    assert_eq!(global_conf.test.debug, true);
}
