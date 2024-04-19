use human_panic::setup_panic;
use log::error;
use std::{boxed::Box, panic};

fn hook(panic_info: &panic::PanicInfo) {
    if cfg!(debug_assertions) {
        let err_message = format!("panic occurred {:?}", panic_info);
        error!("{}", err_message);
    } else {
        let err_message = match panic_info.payload().downcast_ref::<&str>() {
            Option::Some(&str) => {
                let err_message = format!(
                    "panic info: {:?}, occurred in {:?}",
                    str,
                    panic_info.location()
                );
                err_message
            }
            Option::None => {
                let err_message =
                    format!("panic occurred in {:?}", panic_info.location());
                err_message
            }
        };
        error!("{}", err_message);
    }
}

/// 注册异常处理函数
/// 在panic发出后，在panic运行时之前，触发钩子函数去处理这个panic信息。
/// panic信息被保存在PanicInfo结构体中。
pub fn register_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        hook(panic_info);
    }));
    setup_panic!();
}
