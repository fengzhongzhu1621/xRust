use log::{error, log, Level};

fn main() {
    print_env_logger();
}

fn print_env_logger() {
    env_logger::init();
    let data = (1, "one");
    log!(Level::Error, "Received data: {}, {}", data.0, data.1);

    let private_data = "private";
    log!(target: "app_events", Level::Warn, "App warning:{}, {}, {}", data.0, data.1, private_data);
    error!(target: "app_events", "App warning:{}, {}, {}", data.0, data.1, private_data);
}
