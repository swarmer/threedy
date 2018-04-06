use env_logger;


pub fn run() -> i32 {
    env_logger::init();

    info!("run() called!");

    0
}
