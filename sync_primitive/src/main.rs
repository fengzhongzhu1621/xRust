use sync_primitive::*;

fn main() {
    arc_example();
    arc_example2();

    mutex_example1();
    // mutex_example2_poison();
    mutex_example3_drop();
}
