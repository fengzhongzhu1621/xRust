use log::error;
use std::{boxed::Box, panic};

/// 注册异常处理函数
/// 在panic发出后，在panic运行时之前，触发钩子函数去处理这个panic信息。
/// panic信息被保存在PanicInfo结构体中。
pub fn register_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let err_message = match panic_info.payload().downcast_ref::<&str>() {
            Option::Some(&str) => {
                let err_message = format!("panic {}", str);
                err_message
            }
            Option::None => {
                let err_message = format!("panic unknown");
                err_message
            }
        };

        error!("{}", err_message);
    }));
}
