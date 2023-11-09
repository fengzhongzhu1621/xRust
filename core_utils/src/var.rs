use std::borrow::Cow;
use std::env;

#[derive(Debug)]
pub struct Var<'a> {
    name: Cow<'a, str>,            // 变量名
    default: Option<Cow<'a, str>>, // 默认值
}

impl<'a> Var<'a> {
    /// 新建一个变量,默认值为 None
    pub fn new<E>(name: E) -> Self
    where
        E: Into<Cow<'a, str>>,
    {
        Var { name: name.into(), default: None }
    }

    ///  新建一个变量,默认值为 default
    // name 和 default 都实现了到 Cow<'a, str>的转换
    pub fn new_with_default<E, V>(name: E, default: V) -> Self
    where
        E: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        Var { name: name.into(), default: Some(default.into()) }
    }

    /// 根据名称查询变量的值,不如果不存在取默认值
    pub fn get(&self) -> Option<String> {
        // 从环境变量根据名称获取变量的值,如果环境变量中没有设置,则取默认值
        env::var(&*self.name)
            .ok()
            .or_else(|| self.default.to_owned().map(|v| v.into_owned()))
    }
}
