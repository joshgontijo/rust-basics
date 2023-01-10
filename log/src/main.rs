use log::{error, info, trace, warn, LevelFilter};

fn main() {
    env_logger::builder().filter_level(LevelFilter::Trace).init();

    trace!("Trace log");
    info!("such information {}", "yolo");
    warn!("o_O");
    error!("much error");
}
