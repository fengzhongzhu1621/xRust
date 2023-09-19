use timer_examples::*;

fn main() {
    // 同步定时器
    timer_schedule_with_delay();
    timer_schedule_with_date();
    timer_repeat();

    // 异步定时器
    safina_timer_example();

    ticker_example()
}
