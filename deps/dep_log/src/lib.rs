use log::warn;

#[test]
fn log_demo() {
    env_logger::init();
    warn!("starting up");
}

fn log_temp_demo() {
    env_logger::init();
    let user = "Alice";
    let count = 42;
    // 使用占位符
    warn!("starting up for user: {}, count: {}", user, count);
}
