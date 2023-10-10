use triggered;

pub fn triggered_example() {
    smol::block_on(async {
        let (trigger, listener) = triggered::trigger();

        let task = smol::spawn(async {
            // Blocks until `trigger.trigger()` below
            listener.await;

            println!("Triggered async task");
        });

        // This will make any thread blocked in `Listener::wait()` or async task awaiting the
        // listener continue execution again.
        trigger.trigger();

        let _ = task.await;
    })
}
