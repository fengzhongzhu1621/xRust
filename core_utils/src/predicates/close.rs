use super::case::Case;
use super::color::Palette;
use super::parameter::Parameter;
use super::predicate::Predicate;
use super::product::Product;
use super::reflection::PredicateReflection;
use float_cmp::ApproxEq;
use float_cmp::Ulps;
use std::fmt;

/// 定义一个具有指定误差范围精度的浮点数比较断言
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IsClosePredicate {
    target: f64,            // 需要比较的目标
    epsilon: f64, // 误差范围，可以定义一个小的常量，在比较两个浮点数时，将它们的差值与epsilon进行比较
    ulps: <f64 as Ulps>::U, // 浮点数转换为整数的差值
}

impl IsClosePredicate {
    /// 设置误差范围
    /// Set the amount of error allowed.
    pub fn distance(mut self, distance: <f64 as Ulps>::U) -> Self {
        // 计算误差范围
        self.epsilon = (distance as f64) * ::std::f64::EPSILON;
        // 浮点数转换为整数的差值
        self.ulps = distance;
        self
    }

    pub fn epsilon(mut self, epsilon: f64) -> Self {
        self.epsilon = epsilon;
        self
    }

    pub fn ulps(mut self, ulps: <f64 as Ulps>::U) -> Self {
        self.ulps = ulps;
        self
    }
}

impl Predicate<f64> for IsClosePredicate {
    /// 执行断言，返回断言的结果
    fn eval(&self, variable: &f64) -> bool {
        // 浮点数比较 self.target 和 variable 进行比较
        variable.approx_eq(
            self.target, // 需要比较的值
            float_cmp::F64Margin { epsilon: self.epsilon, ulps: self.ulps }, // 误差范围
        )
    }

    /// 构造函数：返回断言成功的描述信息
    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &f64,
    ) -> Option<Case<'a>> {
        // 执行断言，返回断言的结果
        let actual = self.eval(variable);
        if expected == actual {
            Some(
                Case::new(Some(self), actual)
                    .add_product(Product::new(
                        "actual epsilon",
                        (variable - self.target).abs(), // 浮点数差值
                    ))
                    .add_product(Product::new(
                        "actual ulps",
                        variable.ulps(&self.target).abs(), // 浮点数转换为整数的差值
                    )),
            )
        } else {
            None
        }
    }
}

/// 设置断言的参数
impl PredicateReflection for IsClosePredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Parameter<'a>> + 'a> {
        // 构造断言需要的参数
        let params = vec![
            Parameter::new("epsilon", &self.epsilon),
            Parameter::new("ulps", &self.ulps),
        ];
        Box::new(params.into_iter())
    }
}

/// 打印浮点数比较断言
impl fmt::Display for IsClosePredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 创建一个调色板
        let palette = Palette::new(f.alternate());
        write!(
            f,
            "{} {} {}",
            palette.var("var"),            // 红色粗体
            palette.description("!="),     // 蓝色粗体
            palette.expected(self.target), // 绿色粗体
        )
    }
}

/// 创建一个具有默认误差范围精度的浮点数比较断言对象
pub fn is_close(target: f64) -> IsClosePredicate {
    IsClosePredicate { target, epsilon: 2.0 * ::std::f64::EPSILON, ulps: 2 }
}
