use std::fmt;

// 定义一个可现显示的结点
#[derive(Default)]
pub struct Displayable {
    primary: String,   // {} 的打印内容
    alternate: String, // {:#} 的打印内容
}

impl Displayable {
    pub fn new(display: &dyn std::fmt::Display) -> Self {
        let primary = format!("{}", display);
        let alternate = format!("{:#}", display);
        Self { primary, alternate }
    }
}

impl fmt::Display for Displayable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 判断是否包含 # 标识符
        if f.alternate() {
            self.alternate.fmt(f)
        } else {
            self.primary.fmt(f)
        }
    }
}
