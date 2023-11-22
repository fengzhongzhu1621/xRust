///! CachedBool 使用原子操作确保只执行一次闭包
use core::sync::atomic::AtomicU8;
use core::sync::atomic::Ordering::*;

#[allow(unused)]
#[repr(transparent)]
pub struct CachedBool(AtomicU8);

#[allow(unused)]
impl CachedBool {
    const TRUE: u8 = 1;
    const UNINIT: u8 = 2;
    const INITING: u8 = 3;

    pub const fn new() -> Self {
        CachedBool(AtomicU8::new(Self::UNINIT))
    }

    // 执行闭包并返回闭包的执行结果，只执行一次
    pub fn get_or_init(&self, f: impl FnOnce() -> bool) -> bool {
        // 如果值等于 Self::UNINIT，则修改为 Self::INITING，否则返回错误
        // AcqRel 用于read-modify-write类型的操作，比如compare_and_swap，表明了它同时具有Acquire和Release的语义，读的部分用Acquire Ordering，写的部分用Release Ordering。
        // Acquire 任何读和写操作（不限于对当前原子变量）都不能被reorder到该读操作之前。并且当前线程可以看到另一个线程对同一个原子变量使用Release Ordering写操作之前的所有写操作（不限于对当前原子变量）。
        // Release 任何读和写操作（不限于对当前原子变量）都不能被reorder到该写操作之后。并且所有当前线程中在该原子操作之前的所有写操作（不限于对当前原子变量）都对另一个对同一个原子变量使用Acquire Ordering读操作的线程可见。
        match self.0.compare_exchange(
            Self::UNINIT,
            Self::INITING,
            AcqRel,
            Relaxed, // 不施加任何限制，CPU和编译器可以自由reorder，使用了Relaxed Ordering的原子操作只保证原子性。
        ) {
            // 值为 UNINIT，表示未初始化
            Ok(_) => {
                // compare_exchange 确保只执行一次，当第一次初始化时，执行 f 闭包，并保存闭包的返回结果
                // FnOnce闭包只能被外部调用一次
                let new_value = f();
                self.0.store(
                    new_value as u8, /* false = 0, true = 1 */
                    Release,
                );
                new_value
            }
            // 值为 INITING，正在初始化，闭包正在执行
            Err(Self::INITING) => {
                let mut value;
                while {
                    value = self.0.load(Acquire);
                    value
                } == Self::INITING
                {
                    #[cfg(feature = "std")]
                    std::thread::yield_now();
                }

                value == Self::TRUE
            }
            // 闭包执行完毕且初始化完成
            Err(value) => value == Self::TRUE,
        }
    }
}
